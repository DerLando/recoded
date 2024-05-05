use egui_snarl::ui::PinInfo;

use crate::nodes::NodeDowncast;

use super::{InputNode, NodeInfo, Nodes};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct RangeNode {
    start: f64,
    step: f64,
    pub(crate) count: usize,
}

impl RangeNode {
    pub fn get_numbers(&self) -> impl Iterator<Item = f64> + '_ {
        (0..self.count).map(|i| self.start + self.step * i as f64)
    }
}
/// Need to get fancy, since the node is stored
/// inside of the snarl, so if we bind it to a variable
/// we will have to separate mutable references into
/// the snarls inner storage, which will make borowck cry
fn get_node_mut<'a>(
    snarl: &'a mut egui_snarl::Snarl<Nodes>,
    id: egui_snarl::InPinId,
) -> &'a mut RangeNode {
    RangeNode::try_downcast_mut(&mut snarl[id.node]).expect("Is ok")
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

fn show_start_input(
    pin: &egui_snarl::InPin,
    ui: &mut egui::Ui,
    scale: f32,
    snarl: &mut egui_snarl::Snarl<Nodes>,
) -> PinInfo {
    super::show_number_input("Start", pin, ui, scale, snarl, |id, snarl| {
        &mut get_node_mut(snarl, id).start
    })
}

fn show_step_input(
    pin: &egui_snarl::InPin,
    ui: &mut egui::Ui,
    scale: f32,
    snarl: &mut egui_snarl::Snarl<Nodes>,
) -> PinInfo {
    super::show_number_input("Step", pin, ui, scale, snarl, |id, snarl| {
        &mut get_node_mut(snarl, id).step
    })
}

fn show_count_input(
    pin: &egui_snarl::InPin,
    ui: &mut egui::Ui,
    scale: f32,
    snarl: &mut egui_snarl::Snarl<Nodes>,
) -> PinInfo {
    let info: PinInfo;
    ui.label("Count");
    match &*pin.remotes {
        [] => {
            ui.add(egui::DragValue::new(&mut get_node_mut(snarl, pin.id).count));
            info = PinInfo::square().with_fill(crate::UNCONNECTED_COLOR);
        }
        [remote] => {
            if let Some(value) = snarl[remote.node].try_get_float() {
                get_node_mut(snarl, pin.id).count = value as usize;
                ui.label(super::format_float(value));
                info = PinInfo::square().with_fill(crate::NUMBER_COLOR);
            } else {
                ui.add(egui::DragValue::new(&mut get_node_mut(snarl, pin.id).count));
                info = PinInfo::square().with_fill(crate::UNCONNECTED_COLOR);
            }
        }
        _ => unreachable!(),
    }
    info
}

impl InputNode<Nodes> for RangeNode {
    fn show_input(
        pin: &egui_snarl::InPin,
        ui: &mut egui::Ui,
        scale: f32,
        snarl: &mut egui_snarl::Snarl<Nodes>,
    ) -> PinInfo {
        ui.set_width(120.0 * scale);
        ui.set_height(16.0 * scale);
        ui.with_layout(
            egui::Layout::left_to_right(egui::Align::Min).with_cross_align(egui::Align::Center),
            |ui| {
                ui.add_space(20.0 * scale);
                match pin.id.input {
                    0 => {
                        show_start_input(pin, ui, scale, snarl);
                    }
                    1 => {
                        show_step_input(pin, ui, scale, snarl);
                    }
                    2 => {
                        show_count_input(pin, ui, scale, snarl);
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
