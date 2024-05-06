use egui_snarl::ui::PinInfo;

use crate::{
    nodes::NodeDowncast,
    pins::{IPin, InputPin, OPin, OutputPin},
};

use super::{InputNode, NodeInfo, Nodes};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct RangeNode {
    start_in: InputPin<f64>,
    step_in: InputPin<f64>,
    pub(crate) count_in: InputPin<usize>,
    range_out: OutputPin<f64>,
}

impl RangeNode {
    pub fn recalc(&mut self) {
        let start = self.start_in.value_out().copied().unwrap_or_default();
        let step = self.step_in.value_out().copied().unwrap_or_default();
        let count = self.count_in.value_out().copied().unwrap_or_default();
        self.range_out
            .values_in((0..count).map(|i| start + step * i as f64));
    }
    pub fn get_numbers(&self) -> impl Iterator<Item = &f64> {
        self.range_out.values_out().iter()
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
            start_in: InputPin::default(),
            step_in: InputPin::default(),
            count_in: InputPin::default(),
            range_out: OutputPin::default(),
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
    super::show_input_for_number_pin("Start", pin, ui, scale, snarl, |id, snarl| {
        &mut get_node_mut(snarl, id).start_in
    })
}

fn show_step_input(
    pin: &egui_snarl::InPin,
    ui: &mut egui::Ui,
    scale: f32,
    snarl: &mut egui_snarl::Snarl<Nodes>,
) -> PinInfo {
    super::show_input_for_number_pin("Step", pin, ui, scale, snarl, |id, snarl| {
        &mut get_node_mut(snarl, id).step_in
    })
}

fn show_count_input(
    pin: &egui_snarl::InPin,
    ui: &mut egui::Ui,
    scale: f32,
    snarl: &mut egui_snarl::Snarl<Nodes>,
) -> PinInfo {
    super::show_input_for_number_pin("Count", pin, ui, scale, snarl, |id, snarl| {
        &mut get_node_mut(snarl, id).count_in
    })
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

    fn values_in(&mut self, id: crate::pins::InputPinId, values: &crate::values::Values) {
        todo!()
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

    fn values_out(&self, id: crate::pins::OutputPinId) -> crate::values::Values {
        crate::values::Values::Float(self.range_out.values_out().clone())
    }
}
