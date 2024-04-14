use egui::Ui;
use egui_snarl::{ui::PinInfo, InPin, OutPin, Snarl};

use self::{constant_value::ConstantValueNode, range::RangeNode, sink::SinkNode};

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

pub trait NodeDowncast {
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

pub trait NodeInfo {
    fn inputs() -> usize;
    fn outputs() -> usize;
    fn title() -> String;
}

pub trait InputNode<T> {
    fn show_input(pin: &InPin, ui: &mut Ui, scale: f32, snarl: &mut Snarl<T>) -> PinInfo;
}

pub trait OutputNode<T> {
    fn show_output(pin: &OutPin, ui: &mut Ui, scale: f32, snarl: &mut Snarl<T>) -> PinInfo;
}

pub mod range {
    use std::any::TypeId;

    use egui_snarl::ui::PinInfo;

    use crate::nodes::NodeDowncast;

    use super::{InputNode, NodeInfo, Nodes};

    #[derive(serde::Deserialize, serde::Serialize)]
    pub struct RangeNode {
        start: f64,
        step: f64,
        count: usize,
    }

    impl RangeNode {
        pub fn get_numbers(&self) -> impl Iterator<Item = f64> + '_ {
            (0..self.count).map(|i| self.start + self.step * i as f64)
        }
    }

    impl super::Node for RangeNode {}

    impl super::NodeDowncast for RangeNode {
        fn try_downcast(from: &Nodes) -> Option<&Self> {
            match from {
                Nodes::Range(node) => Some(node),
                _ => None,
            }
        }

        fn try_downcast_mut(from: &mut Nodes) -> Option<&mut Self> {
            match from {
                Nodes::Range(node) => Some(node),
                _ => None,
            }
        }
    }

    impl Default for RangeNode {
        fn default() -> Self {
            Self {
                start: 0.0,
                step: 1.0,
                count: 10,
            }
        }
    }

    impl NodeInfo for RangeNode {
        fn inputs() -> usize {
            3
        }

        fn outputs() -> usize {
            1
        }

        fn title() -> String {
            "Range".to_string()
        }
    }

    impl InputNode<Nodes> for RangeNode {
        fn show_input(
            pin: &egui_snarl::InPin,
            ui: &mut egui::Ui,
            scale: f32,
            snarl: &mut egui_snarl::Snarl<Nodes>,
        ) -> PinInfo {
            /// Need to get fancy, since the node is stored
            /// inside of the snarl, so if we bind it to a variable
            /// we will have to separate mutable references into
            /// the snarls inner storage, which will make borowck cry
            fn get_node_mut<'a>(
                snarl: &'a mut egui_snarl::Snarl<Nodes>,
                pin: &'a egui_snarl::InPin,
            ) -> &'a mut RangeNode {
                RangeNode::try_downcast_mut(&mut snarl[pin.id.node]).expect("Is ok")
            }
            ui.set_width(120.0 * scale);
            ui.set_height(16.0 * scale);
            ui.with_layout(
                egui::Layout::left_to_right(egui::Align::Min).with_cross_align(egui::Align::Center),
                |ui| {
                    ui.add_space(20.0 * scale);
                    match pin.id.input {
                        0 => {
                            ui.label("Start");
                            match &*pin.remotes {
                                [] => {
                                    ui.add(egui::DragValue::new(
                                        &mut get_node_mut(snarl, pin).start,
                                    ));
                                }
                                [remote] => {
                                    if let Some(value) = snarl[remote.node].try_get_float() {
                                        ui.label(super::format_float(value));
                                    } else {
                                        ui.add(egui::DragValue::new(
                                            &mut get_node_mut(snarl, pin).start,
                                        ));
                                    }
                                }
                                _ => unreachable!(),
                            }
                        }
                        1 => {
                            ui.add(egui::Label::new("Step"));
                            ui.add(egui::DragValue::new(&mut get_node_mut(snarl, pin).step));
                        }
                        2 => {
                            ui.add(egui::Label::new("Count"));
                            ui.add(egui::DragValue::new(&mut get_node_mut(snarl, pin).count));
                        }
                        _ => unreachable!(),
                    };
                },
            );
            PinInfo::square().with_fill(crate::NUMBER_COLOR)
        }
    }

    impl super::OutputNode<super::Nodes> for RangeNode {
        fn show_output(
            pin: &egui_snarl::OutPin,
            ui: &mut egui::Ui,
            scale: f32,
            snarl: &mut egui_snarl::Snarl<super::Nodes>,
        ) -> PinInfo {
            PinInfo::square().with_fill(crate::NUMBER_COLOR)
        }
    }
}

pub mod constant_value {
    use egui::Ui;
    use egui_snarl::{ui::PinInfo, OutPin};

    use super::NodeInfo;

    #[derive(Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
    pub enum NaturalConstant {
        Pi,
        Tau,
        /// Golden ratio
        Phi,
        /// Golden angle
        Rho,
    }

    impl NaturalConstant {
        fn value(&self) -> f64 {
            match self {
                NaturalConstant::Pi => std::f64::consts::PI,
                NaturalConstant::Tau => std::f64::consts::TAU,
                NaturalConstant::Phi => (1.0 + 5.0_f64.sqrt()) / 2.0,
                NaturalConstant::Rho => std::f64::consts::PI * (3.0 - 5.0_f64.sqrt()),
            }
        }
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct ConstantValueNode {
        value: NaturalConstant,
    }

    impl Default for ConstantValueNode {
        fn default() -> Self {
            Self {
                value: NaturalConstant::Pi,
            }
        }
    }

    impl NodeInfo for ConstantValueNode {
        fn inputs() -> usize {
            0
        }

        fn outputs() -> usize {
            1
        }

        fn title() -> String {
            "Constant".to_string()
        }
    }

    impl ConstantValueNode {
        pub fn number_out(&self) -> f64 {
            self.value.value()
        }

        pub fn show_output(&mut self, ui: &mut Ui) -> PinInfo {
            egui::ComboBox::from_label("Select one")
                .selected_text(format!("{:?}", &mut self.value))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.value, NaturalConstant::Pi, "Pi");
                    ui.selectable_value(&mut self.value, NaturalConstant::Tau, "Tau");
                    ui.selectable_value(&mut self.value, NaturalConstant::Phi, "Phi");
                    ui.selectable_value(&mut self.value, NaturalConstant::Rho, "Rho");
                });
            PinInfo::square().with_fill(crate::NUMBER_COLOR)
        }
    }
}

pub mod sink {
    use egui::Ui;
    use egui_snarl::{ui::PinInfo, InPin, Snarl};

    use super::{NodeInfo, Nodes};

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct SinkNode;

    impl NodeInfo for SinkNode {
        fn inputs() -> usize {
            1
        }

        fn outputs() -> usize {
            0
        }

        fn title() -> String {
            "Sink".to_string()
        }
    }
    pub fn format_float(value: f64) -> String {
        let value = (value * 1000.0).round() / 1000.0;
        format!("{}", value)
    }
    pub fn show_input(pin: &InPin, ui: &mut Ui, scale: f32, snarl: &Snarl<Nodes>) -> PinInfo {
        // TODO: Probably there is more to display than numbers
        match &*pin.remotes {
            [] => {
                ui.label("None");
                PinInfo::circle().with_fill(crate::UNCONNECTED_COLOR)
            }
            [remote] => match &snarl[remote.node] {
                Nodes::ConstantValueNode(value) => {
                    ui.label(format_float(value.number_out()));
                    PinInfo::square().with_fill(crate::NUMBER_COLOR)
                }
                Nodes::Range(node) => {
                    egui::ScrollArea::vertical()
                        .max_height(30.0 * scale)
                        .show(ui, |ui| {
                            ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                                for number in node.get_numbers() {
                                    ui.label(super::format_float(number));
                                }
                            })
                        });
                    PinInfo::square().with_fill(crate::NUMBER_COLOR)
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}
