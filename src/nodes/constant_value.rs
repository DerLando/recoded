use egui::Ui;
use egui_snarl::{ui::PinInfo, OutPin};

use crate::values::Values;

use super::{Node, NodeInfo, Nodes};

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
}

impl super::NodeDowncast for ConstantValueNode {
    fn try_downcast(from: &Nodes) -> Option<&Self> {
        todo!()
    }

    fn try_downcast_mut(from: &mut Nodes) -> Option<&mut Self> {
        match from {
            Nodes::ConstantValueNode(node) => Some(node),
            _ => None,
        }
    }
}

impl super::OutputNode<Nodes> for ConstantValueNode {
    fn show_output(
        pin: &OutPin,
        ui: &mut Ui,
        scale: f32,
        snarl: &mut egui_snarl::Snarl<Nodes>,
    ) -> PinInfo {
        let response = egui::ComboBox::from_label("Select one")
            .selected_text(format!(
                "{:?}",
                &mut super::get_node_mut::<Self>(snarl, pin.id.node).value
            ))
            .show_ui(ui, |ui| {
                if ui
                    .selectable_value(
                        &mut super::get_node_mut::<Self>(snarl, pin.id.node).value,
                        ConstantValue::Pi,
                        "Pi",
                    )
                    .clicked()
                {
                    crate::solver::solve_starting_from(pin.id.node.into(), snarl)
                };
                if ui
                    .selectable_value(
                        &mut super::get_node_mut::<Self>(snarl, pin.id.node).value,
                        ConstantValue::Tau,
                        "Tau",
                    )
                    .clicked()
                {
                    crate::solver::solve_starting_from(pin.id.node.into(), snarl)
                };

                if ui
                    .selectable_value(
                        &mut super::get_node_mut::<Self>(snarl, pin.id.node).value,
                        ConstantValue::Phi,
                        "Phi",
                    )
                    .clicked()
                {
                    crate::solver::solve_starting_from(pin.id.node.into(), snarl)
                };

                if ui
                    .selectable_value(
                        &mut super::get_node_mut::<Self>(snarl, pin.id.node).value,
                        ConstantValue::Rho,
                        "Rho",
                    )
                    .clicked()
                {
                    crate::solver::solve_starting_from(pin.id.node.into(), snarl)
                };

                if ui
                    .selectable_value(
                        &mut super::get_node_mut::<Self>(snarl, pin.id.node).value,
                        ConstantValue::Custom,
                        "Custom",
                    )
                    .clicked()
                {
                    crate::solver::solve_starting_from(pin.id.node.into(), snarl)
                };
            });
        match &mut super::get_node_mut::<Self>(snarl, pin.id.node).value {
            ConstantValue::Custom => {
                if ui
                    .add(egui::DragValue::new(
                        &mut super::get_node_mut::<Self>(snarl, pin.id.node).value_overwrite,
                    ))
                    .changed()
                {
                    crate::solver::solve_starting_from(pin.id.node.into(), snarl);
                }
            }
            _ => (),
        }
        PinInfo::square().with_fill(crate::NUMBER_COLOR)
    }

    fn values_out(&self, _id: crate::pins::OutputPinId) -> crate::values::Values {
        Values::Float(vec![self.number_out()])
    }
}
