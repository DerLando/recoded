use egui::Ui;
use egui_snarl::{ui::PinInfo, OutPin};

use super::{Node, NodeInfo};

#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ConstantValue {
    Pi,
    Tau,
    /// Golden ratio
    Phi,
    /// Golden angle
    Rho,
    Custom,
}

impl ConstantValue {
    fn value(&self) -> f64 {
        match self {
            ConstantValue::Pi => std::f64::consts::PI,
            ConstantValue::Tau => std::f64::consts::TAU,
            ConstantValue::Phi => (1.0 + 5.0_f64.sqrt()) / 2.0,
            ConstantValue::Rho => std::f64::consts::PI * (3.0 - 5.0_f64.sqrt()),
            ConstantValue::Custom => 0.0,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ConstantValueNode {
    value: ConstantValue,
    value_overwrite: f64,
}

impl Default for ConstantValueNode {
    fn default() -> Self {
        Self {
            value: ConstantValue::Pi,
            value_overwrite: 0.0,
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
        match self.value {
            ConstantValue::Custom => self.value_overwrite,
            _ => self.value.value(),
        }
    }

    pub fn set_value(&mut self, value: f64) {}

    pub fn show_output(&mut self, ui: &mut Ui) -> PinInfo {
        egui::ComboBox::from_label("Select one")
            .selected_text(format!("{:?}", &mut self.value))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.value, ConstantValue::Pi, "Pi");
                ui.selectable_value(&mut self.value, ConstantValue::Tau, "Tau");
                ui.selectable_value(&mut self.value, ConstantValue::Phi, "Phi");
                ui.selectable_value(&mut self.value, ConstantValue::Rho, "Rho");
                ui.selectable_value(&mut self.value, ConstantValue::Custom, "Custom");
            });
        match self.value {
            ConstantValue::Custom => {
                ui.add(egui::DragValue::new(&mut self.value_overwrite));
            }
            _ => (),
        }
        PinInfo::square().with_fill(crate::NUMBER_COLOR)
    }
}
