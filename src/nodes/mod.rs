use egui::Ui;
use egui_snarl::{ui::PinInfo, InPin, OutPin, Snarl};

use crate::shapes::Shapes;

use self::{
    canvas::CanvasNode, circle::CircleNode, constant_value::ConstantValueNode, point::PointNode,
    range::RangeNode, repeat::RepeatShapeNode, sink::SinkNode,
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
    RepeatShape(repeat::RepeatShapeNode),
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

pub fn show_point_input(
    title: impl AsRef<str>,
    pin: &InPin,
    ui: &mut Ui,
    scale: f32,
    snarl: &mut Snarl<Nodes>,
    update_fn: impl FnOnce(egui_snarl::InPinId, &mut Snarl<Nodes>) -> &mut piet::kurbo::Point,
) -> PinInfo {
    ui.label(title.as_ref());
    match &*pin.remotes {
        [] => PinInfo::square().with_fill(crate::POINT_COLOR),
        [remote] => {
            if let Some(value) = snarl[remote.node].try_get_point() {
                *(update_fn(pin.id, snarl)) = value;
                ui.label(format_point(value));
                PinInfo::square().with_fill(crate::POINT_COLOR)
            } else {
                PinInfo::square().with_fill(crate::POINT_COLOR)
            }
        }
        _ => unreachable!(),
    }
}

pub fn show_shape_input(
    title: impl AsRef<str>,
    pin: &InPin,
    ui: &mut Ui,
    scale: f32,
    snarl: &mut Snarl<Nodes>,
    update_fn: impl FnOnce(egui_snarl::InPinId, &mut Snarl<Nodes>) -> &mut Shapes,
) -> PinInfo {
    ui.label(title.as_ref());
    match &*pin.remotes {
        [] => PinInfo::square().with_fill(crate::SHAPE_COLOR),
        [remote] => {
            if let Some(value) = snarl[remote.node].try_get_shape() {
                *(update_fn(pin.id, snarl)) = value;
                PinInfo::square().with_fill(crate::SHAPE_COLOR)
            } else {
                PinInfo::square().with_fill(crate::SHAPE_COLOR)
            }
        }
        _ => unreachable!(),
    }
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
            Self::RepeatShape(_) => RepeatShapeNode::inputs(),
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
            Self::RepeatShape(_) => RepeatShapeNode::outputs(),
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
            Self::RepeatShape(_) => RepeatShapeNode::title(),
        }
    }
    pub fn try_get_float(&self) -> Option<f64> {
        match self {
            Self::ConstantValueNode(node) => Some(node.number_out()),
            Self::Range(node) => node.get_numbers().next(),
            _ => None,
        }
    }
    pub fn try_get_point(&self) -> Option<piet::kurbo::Point> {
        match self {
            Self::Point(node) => Some(node.point_out()),
            _ => None,
        }
    }

    pub fn try_get_shape(&self) -> Option<Shapes> {
        match self {
            Self::Circle(node) => Some(node.shape_out()),
            Self::RepeatShape(node) => Some(node.value_out()),
            _ => None,
        }
    }

    pub fn try_get_shapes(&self) -> Option<Vec<Shapes>> {
        match self {
            Self::RepeatShape(node) => Some(node.values_out().collect::<Vec<_>>()),
            Self::Circle(node) => Some(node.values_out().collect::<Vec<_>>()),
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
}

pub trait OutputNode<T>: Node {
    fn show_output(pin: &OutPin, ui: &mut Ui, scale: f32, snarl: &mut Snarl<T>) -> PinInfo;
}

pub trait NumberEmitterNode {
    fn try_get_number(&self) -> f64;
    fn try_get_numbers(&self) -> impl Iterator<Item = f64> + '_;
    // TODO: Maybe something to broadcast numbers?
    // Or, alternatively, nodes can decide that on their inputs
    // themselfes
}
pub trait EmitterNode<T> {
    fn value_out(&self) -> T;
    fn values_out(&self) -> impl Iterator<Item = T> + '_;
}
