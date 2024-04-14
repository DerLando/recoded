use egui_snarl::ui::PinInfo;

use crate::shapes::Shapes;

#[derive(serde::Serialize, serde::Deserialize, Default, Debug)]
pub struct CircleNode {
    circle: piet::kurbo::Circle,
}

impl CircleNode {
    pub fn radius_mut(&mut self) -> &mut f64 {
        &mut self.circle.radius
    }
    pub fn center_mut(&mut self) -> &mut piet::kurbo::Point {
        &mut self.circle.center
    }
    pub fn circle_out(&self) -> &piet::kurbo::Circle {
        &self.circle
    }
    pub fn shape_out(&self) -> Shapes {
        Shapes::Circle(self.circle)
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
                super::get_node_mut::<Self>(snarl, id.node).center_mut()
            }),
            1 => super::show_number_input("Radius", pin, ui, scale, snarl, |id, snarl| {
                super::get_node_mut::<Self>(snarl, id.node).radius_mut()
            }),
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
}
