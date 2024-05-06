use egui_snarl::ui::PinInfo;

use crate::{
    pins::{IPin, InputPin, OPin, OutputPin},
    shapes::Shapes,
};

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct CircleNode {
    center_in: InputPin<piet::kurbo::Point>,
    radius_in: InputPin<f64>,
    circle_out: OutputPin<piet::kurbo::Circle>,
}

impl CircleNode {
    pub fn recalc(&mut self) {
        self.circle_out.values_in(
            self.center_in
                .values_out()
                .iter()
                .zip(self.radius_in.values_out())
                .map(|(center, radius)| piet::kurbo::Circle::new(*center, *radius)),
        )
    }
    pub fn shapes_out(&self) -> impl Iterator<Item = Shapes> {
        self.circle_out
            .values_out()
            .clone()
            .into_iter()
            .map(|circle| Shapes::Circle(circle))
    }
}

impl super::Node for CircleNode {}
impl super::NodeInfo for CircleNode {
    fn inputs() -> usize {
        2
    }

    fn outputs() -> usize {
        1
    }

    fn title() -> String {
        "Circle".to_string()
    }
}
impl super::NodeDowncast for CircleNode {
    fn try_downcast(from: &super::Nodes) -> Option<&Self> {
        todo!()
    }

    fn try_downcast_mut(from: &mut super::Nodes) -> Option<&mut Self> {
        match from {
            super::Nodes::Circle(node) => Some(node),
            _ => None,
        }
    }
}
impl super::InputNode<super::Nodes> for CircleNode {
    fn show_input(
        pin: &egui_snarl::InPin,
        ui: &mut egui::Ui,
        scale: f32,
        snarl: &mut egui_snarl::Snarl<super::Nodes>,
    ) -> egui_snarl::ui::PinInfo {
        match pin.id.input {
            0 => super::show_point_input("Center", pin, ui, scale, snarl, |id, snarl| {
                &mut super::get_node_mut::<Self>(snarl, id.node).center_in
            }),
            1 => super::show_input_for_number_pin("Radius", pin, ui, scale, snarl, |id, snarl| {
                &mut super::get_node_mut::<Self>(snarl, id.node).radius_in
            }),
            _ => unreachable!(),
        }
    }

    fn values_in(&mut self, id: crate::pins::InputPinId, values: &crate::values::Values) {
        match id.0 {
            0 => match values {
                crate::values::Values::Point(pts) => self.center_in.values_in(pts.iter().cloned()),
                _ => (),
            },
            1 => match values {
                crate::values::Values::Float(numbers) => {
                    self.radius_in.values_in(numbers.iter().cloned())
                }
                crate::values::Values::Int(numbers) => {
                    self.radius_in.values_in(numbers.iter().cloned())
                }
                _ => (),
            },
            _ => unreachable!(),
        }
    }
}

impl super::OutputNode<super::Nodes> for CircleNode {
    fn show_output(
        pin: &egui_snarl::OutPin,
        ui: &mut egui::Ui,
        scale: f32,
        snarl: &mut egui_snarl::Snarl<super::Nodes>,
    ) -> egui_snarl::ui::PinInfo {
        ui.label("Circle");
        PinInfo::triangle().with_fill(crate::SHAPE_COLOR)
    }

    fn values_out(&self, id: crate::pins::OutputPinId) -> crate::values::Values {
        crate::values::Values::Circle(self.circle_out.values_out().clone())
    }
}
