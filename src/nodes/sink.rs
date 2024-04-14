use egui::Ui;
use egui_snarl::{ui::PinInfo, InPin, Snarl};

use super::{Node, NodeInfo, Nodes};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SinkNode;

impl Node for SinkNode {}

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
            Nodes::Point(node) => {
                ui.label(format!("{:?}", node.point_out()));
                PinInfo::circle().with_fill(crate::POINT_COLOR)
            }
            Nodes::Circle(node) => {
                ui.label(format!("{:?}", node.circle_out()));
                PinInfo::triangle().with_fill(crate::SHAPE_COLOR)
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}
