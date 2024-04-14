use egui::Ui;
use egui_snarl::{ui::PinInfo, InPin, OutPin, Snarl};

use self::{constant_value::ConstantValueNode, range::RangeNode, sink::SinkNode};

pub mod range;

pub mod constant_value;

pub mod sink;

/// Main enum containing all node types
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Nodes {
    ConstantValueNode(constant_value::ConstantValueNode),
    Sink(sink::SinkNode),
    Range(range::RangeNode),
}
pub fn format_float(value: f64) -> String {
    let value = (value * 1000.0).round() / 1000.0;
    format!("{}", value)
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
        }
    }
    pub fn outputs(&self) -> usize {
        match self {
            Self::ConstantValueNode(_) => ConstantValueNode::outputs(),
            Self::Sink(_) => SinkNode::outputs(),
            Self::Range(_) => RangeNode::outputs(),
        }
    }
    pub fn title(&self) -> String {
        match self {
            Self::ConstantValueNode(_) => ConstantValueNode::title(),
            Self::Sink(_) => SinkNode::title(),
            Self::Range(_) => RangeNode::title(),
        }
    }
    pub fn try_get_float(&self) -> Option<f64> {
        match self {
            Self::ConstantValueNode(node) => Some(node.number_out()),
            Self::Range(node) => node.get_numbers().next(),
            _ => None,
        }
    }
    pub fn try_downcast<N>(&self) -> Option<&N>
    where
        N: Node,
    {
        todo!()
    }

    pub fn try_downcast_mut<N>(&mut self) -> Option<&mut N>
    where
        N: Node,
    {
        todo!()
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
