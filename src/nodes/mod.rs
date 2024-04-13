#[derive(serde::Serialize, serde::Deserialize)]
pub enum Nodes {
    ConstantValueNode(constant_value::ConstantValueNode),
    Sink(sink::SinkNode),
}

pub mod constant_value {
    use egui::Ui;
    use egui_snarl::{ui::PinInfo, OutPin};

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
                NaturalConstant::Rho => {
                    NaturalConstant::Tau.value()
                        - NaturalConstant::Tau.value() / NaturalConstant::Phi.value()
                }
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
            PinInfo::square()
        }
    }
}

pub mod sink {
    use egui::Ui;
    use egui_snarl::{ui::PinInfo, InPin, Snarl};

    use super::Nodes;

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct SinkNode;
    pub fn format_float(value: f64) -> String {
        let value = (value * 1000.0).round() / 1000.0;
        format!("{}", value)
    }
    pub fn show_input(pin: &InPin, ui: &mut Ui, snarl: &Snarl<Nodes>) -> PinInfo {
        // TODO: Probably there is more to display than numbers
        match &*pin.remotes {
            [] => {
                ui.label("None");
                PinInfo::triangle()
            }
            [remote] => match &snarl[remote.node] {
                Nodes::ConstantValueNode(value) => {
                    ui.label(format_float(value.number_out()));
                    PinInfo::square()
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}
