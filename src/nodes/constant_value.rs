use egui::Ui;
use egui_snarl::{ui::PinInfo, OutPin};

use super::{Node, NodeInfo};

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

impl Node for ConstantValueNode {}

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
