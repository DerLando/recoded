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
struct NodeId(usize);

struct NodeStoreRef<'a, T>
where
    T: std::ops::Index<NodeId, Output = Nodes>,
{
    inner: &'a T,
}

impl<'a, T> NodeStoreRef<'a, T>
where
    T: std::ops::Index<NodeId, Output = Nodes>,
{
    pub fn downstream_node(&self, out_id: OutputPinId) -> Option<NodeId> {
        todo!()
    }
}

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
        todo!()
    }
}
impl<'a> std::ops::IndexMut<NodeId> for egui_snarl::Snarl<Nodes> {
    fn index_mut(&mut self, index: NodeId) -> &mut Self::Output {
        todo!()
    }
}
impl<'a> From<&'a egui_snarl::Snarl<Nodes>> for NodeStoreRef<'a, egui_snarl::Snarl<Nodes>> {
    fn from(value: &'a egui_snarl::Snarl<Nodes>) -> Self {
        todo!()
    }
}
impl<'a> From<egui_snarl::Snarl<Nodes>> for NodeStoreRef<'a, egui_snarl::Snarl<Nodes>> {
    fn from(value: egui_snarl::Snarl<Nodes>) -> Self {
        todo!()
    }
}
impl<'a> From<&'a mut egui_snarl::Snarl<Nodes>> for NodeStoreMut<'a, egui_snarl::Snarl<Nodes>> {
    fn from(value: &'a mut egui_snarl::Snarl<Nodes>) -> Self {
        todo!()
    }
}
impl<'a> From<egui_snarl::Snarl<Nodes>> for NodeStoreMut<'a, egui_snarl::Snarl<Nodes>> {
    fn from(value: egui_snarl::Snarl<Nodes>) -> Self {
        todo!()
    }
}

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

    fn mark_nodes_from<'a, T>(&mut self, store: &T, node_id: NodeId)
    where
        T: std::ops::Index<NodeId, Output = Nodes> + 'a,
    {
        let store = NodeStoreRef { inner: store };
        self.mark_node_inner(&store, node_id, 0);
    }

    fn mark_node_inner<'a, T>(&mut self, store: &NodeStoreRef<'_, T>, node_id: NodeId, rank: usize)
    where
        T: std::ops::Index<NodeId, Output = Nodes> + 'a,
    {
        self.store_node_rank(node_id, rank);

        let node = &store.inner[node_id];
        for out_pin_id in node.out_ids() {
            if let Some(downstream) = &store.downstream_node(out_pin_id) {
                self.mark_node_inner(store, *downstream, rank + 1)
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

    pub fn get_rank(&self, node_id: NodeId) -> Option<usize> {
        self.rank_by_node.get(&node_id).copied()
    }
}

pub struct Solver {
    nodes_ids: Vec<NodeId>,
    connections: Vec<OutgoingConnections>,
    dirty_ranks: HashMap<usize, Vec<NodeId>>,
    dirty_ranks_by_node: HashMap<NodeId, usize>,
}

impl Solver {}

struct OutgoingConnections {
    node_id: NodeId,
    pin_id: OutputPinId,
}

impl OutgoingConnections {
    fn get_connections<'a, S, T>(&self, store: &S) -> impl Iterator<Item = (NodeId, InputPinId)>
    where
        S: Into<NodeStoreRef<'a, T>>,
        T: std::ops::Index<NodeId, Output = Nodes> + 'a,
    {
        (0..1).into_iter().map(|_| (NodeId(0), InputPinId(0)))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_api() {
        let mut snarl: egui_snarl::Snarl<Nodes> = egui_snarl::Snarl::new();
        // Solver::mark_node_for_solve(&mut snarl, NodeId(0));
        let connections = OutgoingConnections {
            node_id: todo!(),
            pin_id: todo!(),
        };
        connections.get_connections(&snarl);
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
}
