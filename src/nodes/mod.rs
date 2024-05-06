use egui::Ui;
use egui_snarl::{ui::PinInfo, InPin, OutPin, Snarl};

use crate::{
    pins::{IPin, InputPin, InputPinId, OPin, OutputPinId},
    shapes::Shapes,
    values::Values,
};

use self::{
    canvas::CanvasNode, circle::CircleNode, constant_value::ConstantValueNode, point::PointNode,
    range::RangeNode, sink::SinkNode,
};

pub mod canvas;
pub mod circle;
pub mod constant_value;
pub mod point;
pub mod range;
pub mod repeat;
pub mod sink;

/// Main enum containing all node types
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Nodes {
    ConstantValueNode(constant_value::ConstantValueNode),
    Sink(sink::SinkNode),
    Range(range::RangeNode),
    Point(point::PointNode),
    Circle(circle::CircleNode),
    Canvas(canvas::CanvasNode),
}

impl Nodes {
    /// Recalculate all outputs of the node
    /// and mark all it's inputs as clean
    pub fn solve(&mut self) {
        match self {
            Nodes::ConstantValueNode(_) => (),
            Nodes::Sink(_) => (),
            Nodes::Range(node) => node.recalc(),
            Nodes::Point(node) => node.recalc(),
            Nodes::Circle(node) => node.recalc(),
            Nodes::Canvas(node) => node.recalc(),
        }
    }

    pub fn values_in(&mut self, id: InputPinId, values: &crate::values::Values) {
        match self {
            Nodes::ConstantValueNode(_) => unreachable!(),
            Nodes::Sink(_) => (),
            Nodes::Range(node) => node.values_in(id, values),
            Nodes::Point(node) => node.values_in(id, values),
            Nodes::Circle(node) => node.values_in(id, values),
            Nodes::Canvas(node) => node.values_in(id, values),
        }
    }

    pub fn values_out(&self, id: OutputPinId) -> crate::values::Values {
        match self {
            Nodes::ConstantValueNode(node) => Values::Float(vec![node.number_out()]),
            Nodes::Sink(_) => unreachable!(),
            Nodes::Range(node) => node.values_out(OutputPinId::default()),
            Nodes::Point(node) => node.values_out(OutputPinId::default()),
            Nodes::Circle(node) => node.values_out(id),
            Nodes::Canvas(_) => unreachable!(),
        }
    }

    /// PinIds are just raw indices, not uuids, so we can
    /// just enumerate the count :)
    pub fn out_ids(&self) -> impl Iterator<Item = OutputPinId> {
        (0..self.outputs()).into_iter().map(|n| OutputPinId(n))
    }
}
pub fn format_float(value: f64) -> String {
    let value = (value * 1000.0).round() / 1000.0;
    format!("{}", value)
}
pub fn format_point(value: piet::kurbo::Point) -> String {
    let x = (value.x * 1000.0).round() / 1000.0;
    let y = (value.y * 1000.0).round() / 1000.0;
    format!("({};{})", x, y)
}
pub fn show_number_input(
    title: impl AsRef<str>,
    pin: &InPin,
    ui: &mut Ui,
    scale: f32,
    snarl: &mut Snarl<Nodes>,
    update_fn: impl FnOnce(egui_snarl::InPinId, &mut Snarl<Nodes>) -> &mut f64,
) -> PinInfo {
    ui.label(title.as_ref());
    match &*pin.remotes {
        [] => {
            ui.add(egui::DragValue::new(update_fn(pin.id, snarl)));
            PinInfo::square().with_fill(crate::NUMBER_COLOR)
        }
        [remote] => {
            if let Some(value) = snarl[remote.node].try_get_float() {
                *(update_fn(pin.id, snarl)) = value;
                ui.label(format_float(value));
                PinInfo::square().with_fill(crate::NUMBER_COLOR)
            } else {
                ui.add(egui::DragValue::new(update_fn(pin.id, snarl)));
                PinInfo::square().with_fill(crate::NUMBER_COLOR)
            }
        }
        _ => unreachable!(),
    }
}

pub fn show_input_for_number_pin<N>(
    title: impl AsRef<str>,
    pin: &InPin,
    ui: &mut Ui,
    scale: f32,
    snarl: &mut Snarl<Nodes>,
    update_fn: impl FnOnce(egui_snarl::InPinId, &mut Snarl<Nodes>) -> &mut InputPin<N>,
) -> PinInfo
where
    N: eframe::emath::Numeric,
{
    fn add_dragvalue_for_pin<N: eframe::emath::Numeric>(
        ui: &mut Ui,
        in_pin: &mut InputPin<N>,
    ) -> bool {
        let mut changed = false;
        let drag_value = egui::DragValue::from_get_set(|v| {
            if let Some(v) = v {
                in_pin.value_in(eframe::emath::Numeric::from_f64(v));
                changed = true;
            }
            in_pin.value_out().map(|n| n.to_f64()).unwrap_or_default()
        });
        ui.add(drag_value);
        changed
    }

    ui.label(title.as_ref());
    match &*pin.remotes {
        [] => {
            if add_dragvalue_for_pin(ui, update_fn(pin.id, snarl)) {
                crate::solver::solve_starting_from(pin.id.node.into(), snarl)
            }
            PinInfo::square().with_fill(crate::NUMBER_COLOR)
        }
        [remote] => {
            if let Some(value) = snarl[remote.node].try_get_floats() {
                (update_fn(pin.id, snarl)).values_in(value.iter().map(|v| N::from_f64(*v)));
                // ui.label(format_float(value));
                PinInfo::square().with_fill(crate::NUMBER_COLOR)
            } else {
                add_dragvalue_for_pin(ui, update_fn(pin.id, snarl));
                PinInfo::square().with_fill(crate::NUMBER_COLOR)
            }
        }
        _ => unreachable!(),
    }
}

pub fn show_point_input(
    title: impl AsRef<str>,
    pin: &InPin,
    ui: &mut Ui,
    scale: f32,
    snarl: &mut Snarl<Nodes>,
    update_fn: impl FnOnce(egui_snarl::InPinId, &mut Snarl<Nodes>) -> &mut InputPin<piet::kurbo::Point>,
) -> PinInfo {
    ui.label(title.as_ref());
    PinInfo::square().with_fill(crate::POINT_COLOR)
}

pub fn show_shape_input(
    title: impl AsRef<str>,
    pin: &InPin,
    ui: &mut Ui,
    scale: f32,
    snarl: &mut Snarl<Nodes>,
    update_fn: impl FnOnce(egui_snarl::InPinId, &mut Snarl<Nodes>) -> &mut InputPin<Shapes>,
) -> PinInfo {
    ui.label(title.as_ref());
    PinInfo::square().with_fill(crate::SHAPE_COLOR)
}

// pub fn show_shapes_input(
//     title: impl AsRef<str>,
//     pin: &InPin,
//     ui: &mut Ui,
//     scale: f32,
//     snarl: &mut Snarl<Nodes>,
//     update_fn: impl FnOnce(egui_snarl::InPinId, &mut Snarl<Nodes>) -> &mut Vec<Shapes>,
// ) -> PinInfo {
//     ui.label(title.as_ref());
//     match &*pin.remotes {
//         [] => PinInfo::square().with_fill(crate::SHAPE_COLOR),
//         [remote] => {
//             if let Some(value) = snarl[remote.node].try_get_point() {
//                 *(update_fn(pin.id, snarl)) = value;
//                 ui.label(format_point(value));
//                 PinInfo::square().with_fill(crate::SHAPE_COLOR)
//             } else {
//                 PinInfo::square().with_fill(crate::SHAPE_COLOR)
//             }
//         }
//         _ => unreachable!(),
//     }
// }

fn get_node_mut<'a, N>(snarl: &'a mut egui_snarl::Snarl<Nodes>, id: egui_snarl::NodeId) -> &'a mut N
where
    N: NodeDowncast,
{
    N::try_downcast_mut(&mut snarl[id]).expect("Is ok")
}

/// Marker trait for node structs
pub trait Node {}

/// Helper trait for downcasting from [`Nodes`] instances
/// to the concete node types stored inside
pub trait NodeDowncast: Node {
    fn try_downcast(from: &Nodes) -> Option<&Self>;
    fn try_downcast_mut(from: &mut Nodes) -> Option<&mut Self>;
}

impl Nodes {
    pub fn inputs(&self) -> usize {
        match self {
            Self::ConstantValueNode(_) => ConstantValueNode::inputs(),
            Self::Sink(_) => SinkNode::inputs(),
            Self::Range(_) => RangeNode::inputs(),
            Self::Point(_) => PointNode::inputs(),
            Self::Circle(_) => CircleNode::inputs(),
            Self::Canvas(_) => CanvasNode::inputs(),
        }
    }
    pub fn outputs(&self) -> usize {
        match self {
            Self::ConstantValueNode(_) => ConstantValueNode::outputs(),
            Self::Sink(_) => SinkNode::outputs(),
            Self::Range(_) => RangeNode::outputs(),
            Self::Point(_) => PointNode::outputs(),
            Self::Circle(_) => CircleNode::outputs(),
            Self::Canvas(_) => CanvasNode::outputs(),
        }
    }
    pub fn title(&self) -> String {
        match self {
            Self::ConstantValueNode(_) => ConstantValueNode::title(),
            Self::Sink(_) => SinkNode::title(),
            Self::Range(_) => RangeNode::title(),
            Self::Point(_) => PointNode::title(),
            Self::Circle(_) => CircleNode::title(),
            Self::Canvas(_) => CanvasNode::title(),
        }
    }
    pub fn try_get_float(&self) -> Option<f64> {
        match self {
            Self::ConstantValueNode(node) => Some(node.number_out()),
            Self::Range(node) => node.get_numbers().next().cloned(),
            _ => None,
        }
    }
    pub fn try_get_floats(&self) -> Option<Vec<f64>> {
        match self {
            Self::ConstantValueNode(node) => Some(vec![node.number_out()]),
            Self::Range(node) => Some(node.get_numbers().cloned().collect::<Vec<_>>()),
            _ => None,
        }
    }
    pub fn try_get_point(&mut self) -> Option<piet::kurbo::Point> {
        match self {
            Self::Point(node) => Some(node.point_out()),
            _ => None,
        }
    }

    pub fn try_get_shape(&self) -> Option<Shapes> {
        match self {
            Self::Circle(node) => node.shapes_out().next(),
            _ => None,
        }
    }

    pub fn try_get_shapes(&self) -> Option<impl Iterator<Item = Shapes>> {
        match self {
            Self::Circle(node) => Some(node.shapes_out()),
            _ => None,
        }
    }

    pub fn try_get_points(&self) -> Option<Vec<piet::kurbo::Point>> {
        match self {
            Self::Point(node) => Some(node.points_out().map(|pt| *pt).collect::<Vec<_>>()),
            _ => None,
        }
    }
}

/// General info for a node, every [`Node`] implementor
/// should also implement this
pub trait NodeInfo: Node {
    fn inputs() -> usize;
    fn outputs() -> usize;
    fn title() -> String;
}

pub trait InputNode<T>: Node {
    fn show_input(pin: &InPin, ui: &mut Ui, scale: f32, snarl: &mut Snarl<T>) -> PinInfo;
    fn values_in(&mut self, id: InputPinId, values: &Values);
}

pub trait OutputNode<T>: Node {
    fn show_output(pin: &OutPin, ui: &mut Ui, scale: f32, snarl: &mut Snarl<T>) -> PinInfo;
    fn values_out(&self, id: OutputPinId) -> Values;
}
