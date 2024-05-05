use std::collections::HashMap;

use crate::{
    nodes::Nodes,
    pins::{InputPinId, OutputPinId},
};

/// TODO: There should be 2 parts of the solver
/// 1. The `Marker`, traverses the subgraph of changes
/// starting from the first changed node and marks all
/// downstream nodes as changed by flagging their inputd
/// as dirty. It also collects information about the rank
/// of each visited node measured from the starting node
/// and returns that at the end of the marking operation.
/// 2. The `Solver`, traverses the subgraph of changes
/// one rank at a time. This way it is guaranteed we are
/// never computing a node value, which would need to still
/// gather inputs from upstream nodes. After computing a node,
/// All inputs on the node have to be flagged as clean again
/// to prepare for the next solve.
/// Also marker and solver need to operate on the exact same nodestore
/// so we can leave out some internal bounds check, as every marked
/// node also will have to be computed at the change step.
/// For this to work the api needs to take an exclusive reference
/// to a node store and run the whole solving pipeline at once.

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId(usize);

impl From<egui_snarl::NodeId> for NodeId {
    fn from(value: egui_snarl::NodeId) -> Self {
        Self(value.0)
    }
}

impl From<egui_snarl::InPinId> for InputPinId {
    fn from(value: egui_snarl::InPinId) -> Self {
        Self(value.input)
    }
}

impl From<egui_snarl::OutPinId> for OutputPinId {
    fn from(value: egui_snarl::OutPinId) -> Self {
        Self(value.output)
    }
}

fn get_downstream_nodes(
    snarl: &egui_snarl::Snarl<Nodes>,
    node_id: NodeId,
    out_id: OutputPinId,
) -> impl Iterator<Item = (NodeId, InputPinId)> {
    let pin = snarl.out_pin(egui_snarl::OutPinId {
        node: egui_snarl::NodeId(node_id.0),
        output: out_id.0,
    });
    pin.remotes.into_iter().map(|r| (r.node.into(), r.into()))
}

trait DownStreamTopology {
    fn get_downstream_inputs(
        &self,
        node_id: NodeId,
        out_id: OutputPinId,
    ) -> impl Iterator<Item = (NodeId, InputPinId)>;
}

impl DownStreamTopology for egui_snarl::Snarl<Nodes> {
    fn get_downstream_inputs(
        &self,
        node_id: NodeId,
        out_id: OutputPinId,
    ) -> impl Iterator<Item = (NodeId, InputPinId)> {
        get_downstream_nodes(&self, node_id, out_id)
    }
}

struct NodeStoreRef<'a, T>
where
    T: std::ops::Index<NodeId, Output = Nodes> + DownStreamTopology,
{
    inner: &'a T,
}

impl<'a, T> NodeStoreRef<'a, T> where T: std::ops::Index<NodeId, Output = Nodes> + DownStreamTopology
{}

struct NodeStoreMut<'a, T>
where
    T: std::ops::IndexMut<NodeId, Output = Nodes>,
{
    inner: &'a mut T,
}

// This mapping should ideally move to some conversion layer
impl<'a> std::ops::Index<NodeId> for egui_snarl::Snarl<Nodes> {
    type Output = Nodes;

    fn index(&self, index: NodeId) -> &Self::Output {
        let id = egui_snarl::NodeId(index.0);
        &self[id]
    }
}
impl<'a> std::ops::IndexMut<NodeId> for egui_snarl::Snarl<Nodes> {
    fn index_mut(&mut self, index: NodeId) -> &mut Self::Output {
        let id = egui_snarl::NodeId(index.0);
        &mut self[id]
    }
}
impl<'a> From<&'a egui_snarl::Snarl<Nodes>> for NodeStoreRef<'a, egui_snarl::Snarl<Nodes>> {
    fn from(value: &'a egui_snarl::Snarl<Nodes>) -> Self {
        Self { inner: value }
    }
}
// impl<'a> From<egui_snarl::Snarl<Nodes>> for NodeStoreRef<'a, egui_snarl::Snarl<Nodes>> {
//     fn from(value: egui_snarl::Snarl<Nodes>) -> Self {
//         Self { inner: &value }
//     }
// }
impl<'a> From<&'a mut egui_snarl::Snarl<Nodes>> for NodeStoreMut<'a, egui_snarl::Snarl<Nodes>> {
    fn from(value: &'a mut egui_snarl::Snarl<Nodes>) -> Self {
        Self { inner: value }
    }
}
// impl<'a> From<egui_snarl::Snarl<Nodes>> for NodeStoreMut<'a, egui_snarl::Snarl<Nodes>> {
//     fn from(value: egui_snarl::Snarl<Nodes>) -> Self {
//         todo!()
//     }
// }

struct Marker {
    ranks: HashMap<usize, Vec<NodeId>>,
    rank_by_node: HashMap<NodeId, usize>,
}

impl Marker {
    pub fn new() -> Self {
        Self {
            ranks: HashMap::new(),
            rank_by_node: HashMap::new(),
        }
    }

    /// Mark all nodes starting from the given node for solving
    /// TODO: This should also mark the individual input params
    /// so we don't have to fetch data on all input params
    /// when solving. A node with 20 params where only one changed
    /// would be the example where this optimization makes sense
    fn mark_nodes_from<'a, T>(mut self, store: &T, node_id: NodeId) -> HashMap<usize, Vec<NodeId>>
    where
        T: std::ops::Index<NodeId, Output = Nodes> + DownStreamTopology + 'a,
    {
        let store = NodeStoreRef { inner: store };
        self.mark_node_inner(&store, node_id, 0);

        self.ranks
    }

    fn mark_node_inner<'a, T>(&mut self, store: &NodeStoreRef<'_, T>, node_id: NodeId, rank: usize)
    where
        T: std::ops::Index<NodeId, Output = Nodes> + DownStreamTopology + 'a,
    {
        self.store_node_rank(node_id, rank);

        let node = &store.inner[node_id];
        for out_pin_id in node.out_ids() {
            for (downstream_node, _) in store.inner.get_downstream_inputs(node_id, out_pin_id) {
                // TODO: Use the input pin id info here to mark
                // the actual input that needs to recompute
                self.mark_node_inner(store, downstream_node, rank + 1)
            }
        }
    }

    /// Ranks are unique per node and monotonically increasing, so if a node
    /// is already stored for a lower rank, it will be removed
    /// from that rank and stored at the higher rank instead
    fn store_node_rank(&mut self, node_id: NodeId, rank: usize) {
        // Check if the node has already been assigned to a rank before
        if self.rank_by_node.contains_key(&node_id) {
            // Remove the node from the lower rank
            let old_rank = self.rank_by_node[&node_id];
            self.ranks.entry(old_rank).and_modify(|rank_nodes| {
                rank_nodes.remove(
                    rank_nodes
                        .iter()
                        .position(|id| *id == node_id)
                        .expect("Checked inclusion"),
                );
            });
            self.rank_by_node.remove(&node_id);
        }
        // Insert into the ranks tables
        self.ranks
            .entry(rank)
            .and_modify(|nodes| nodes.push(node_id))
            .or_insert(vec![node_id]);
        self.rank_by_node.insert(node_id, rank);
    }

    /// Get the rank of a given node, as measured from the start
    /// node of the change event
    pub fn get_rank(&self, node_id: NodeId) -> Option<usize> {
        self.rank_by_node.get(&node_id).copied()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Vec<NodeId>> {
        self.ranks.values()
    }
}

pub fn solve_starting_from<'a, T>(node_id: NodeId, store: &mut T)
where
    T: std::ops::Index<NodeId, Output = Nodes> + DownStreamTopology + 'a,
{
    todo!()
}

#[cfg(test)]
mod test {
    use crate::nodes::{point::PointNode, range::RangeNode, NodeDowncast};

    use super::*;

    #[test]
    fn test_api() {
        let mut snarl: egui_snarl::Snarl<Nodes> = egui_snarl::Snarl::new();
        // Solver::mark_node_for_solve(&mut snarl, NodeId(0));
    }

    #[test]
    fn noderanks_should_increase_on_conflict() {
        let mut marker = Marker::new();
        let id = NodeId(0);
        marker.store_node_rank(id, 0);
        assert_eq!(0, marker.get_rank(id).unwrap());
        marker.store_node_rank(id, 3);
        assert_eq!(3, marker.get_rank(id).unwrap());
    }

    #[test]
    fn marker_should_mark_ranks() {
        let mut snarl: egui_snarl::Snarl<Nodes> = egui_snarl::Snarl::new();
        let node = super::Nodes::Range(RangeNode::default());
        let range_id = snarl.insert_node(egui::Pos2::ZERO, node);
        let node = super::Nodes::Point(PointNode::default());
        let point_id = snarl.insert_node(egui::Pos2::ZERO, node);
        let range_out_pin = egui_snarl::OutPinId {
            node: range_id,
            output: 0,
        };
        let point_in_pin = egui_snarl::InPinId {
            node: point_id,
            input: 0,
        };
        assert!(snarl.connect(range_out_pin, point_in_pin));

        let range_id: NodeId = range_id.into();
        let point_id: NodeId = point_id.into();
        let marker = Marker::new();
        let ranks = marker.mark_nodes_from(&snarl, range_id);

        assert_eq!(range_id, ranks[&0][0]);
        assert_eq!(point_id, ranks[&1][0]);
    }

    #[test]
    fn solver_should_solve_simple_setup() {
        let mut snarl: egui_snarl::Snarl<Nodes> = egui_snarl::Snarl::new();
        let node = super::Nodes::Range(RangeNode::default());
        let range_id = snarl.insert_node(egui::Pos2::ZERO, node);
        let node = super::Nodes::Point(PointNode::default());
        let point_id = snarl.insert_node(egui::Pos2::ZERO, node);
        let range_out_pin = egui_snarl::OutPinId {
            node: range_id,
            output: 0,
        };
        let point_in_pin = egui_snarl::InPinId {
            node: point_id,
            input: 0,
        };
        assert!(snarl.connect(range_out_pin, point_in_pin));

        RangeNode::try_downcast_mut(snarl.get_node_mut(range_id).unwrap())
            .unwrap()
            .count = 10;

        assert_eq!(
            0,
            snarl
                .get_node(point_id)
                .unwrap()
                .try_get_points()
                .unwrap()
                .len()
        );

        solve_starting_from(NodeId::from(range_id), &mut snarl);
        assert_eq!(
            10,
            snarl
                .get_node(point_id)
                .unwrap()
                .try_get_points()
                .unwrap()
                .len()
        );
    }
}
