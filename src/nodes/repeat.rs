// use egui_snarl::ui::PinInfo;

// use crate::shapes::Shapes;

// #[derive(serde::Serialize, serde::Deserialize, Default)]
// pub struct RepeatShapeNode {
//     shape: Shapes,
//     count: f64,
// }

// impl super::Node for RepeatShapeNode {}
// impl super::NodeInfo for RepeatShapeNode {
//     fn inputs() -> usize {
//         2
//     }

//     fn outputs() -> usize {
//         1
//     }

//     fn title() -> String {
//         "RepeatShape".to_string()
//     }
// }
// impl super::NodeDowncast for RepeatShapeNode {
//     fn try_downcast(from: &super::Nodes) -> Option<&Self> {
//         todo!()
//     }

//     fn try_downcast_mut(from: &mut super::Nodes) -> Option<&mut Self> {
//         match from {
//             super::Nodes::RepeatShape(node) => Some(node),
//             _ => None,
//         }
//     }
// }
// impl super::InputNode<super::Nodes> for RepeatShapeNode {
//     fn show_input(
//         pin: &egui_snarl::InPin,
//         ui: &mut egui::Ui,
//         scale: f32,
//         snarl: &mut egui_snarl::Snarl<super::Nodes>,
//     ) -> egui_snarl::ui::PinInfo {
//         match pin.id.input {
//             0 => super::show_shape_input("Shape", pin, ui, scale, snarl, |id, snarl| {
//                 &mut super::get_node_mut::<Self>(snarl, id.node).shape
//             }),
//             1 => super::show_number_input("Count", pin, ui, scale, snarl, |id, snarl| {
//                 &mut super::get_node_mut::<Self>(snarl, id.node).count
//             }),
//             _ => unreachable!(),
//         }
//     }
// }
// impl super::OutputNode<super::Nodes> for RepeatShapeNode {
//     fn show_output(
//         pin: &egui_snarl::OutPin,
//         ui: &mut egui::Ui,
//         scale: f32,
//         snarl: &mut egui_snarl::Snarl<super::Nodes>,
//     ) -> egui_snarl::ui::PinInfo {
//         ui.label("Shapes");
//         PinInfo::triangle().with_fill(crate::SHAPE_COLOR)
//     }
// }
// impl super::EmitterNode<Shapes> for RepeatShapeNode {
//     fn value_out(&self) -> Shapes {
//         self.shape.clone()
//     }

//     fn values_out(&self) -> impl Iterator<Item = Shapes> + '_ {
//         (0..self.count as usize).map(|_| self.shape.clone())
//     }
// }
