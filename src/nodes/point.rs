use egui_snarl::ui::PinInfo;

use crate::pins::{IPin, InputPin, InputPinId, OPin, OutputPin};

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct PointNode {
    x_in: InputPin<f64>,
    y_in: InputPin<f64>,
    point_out: OutputPin<piet::kurbo::Point>,
}

impl PointNode {
    fn recalc(&mut self) {
        let x = self.x_in.values_out();
        let y = self.y_in.values_out();
        let pts = x
            .iter()
            .zip(y.iter())
            .map(|(x, y)| piet::kurbo::Point::new(*x, *y));
        self.point_out.values_in(pts);
    }
    fn needs_recalc(&self) -> bool {
        self.x_in.is_dirty() || self.y_in.is_dirty()
    }
    pub fn point_out(&self) -> piet::kurbo::Point {
        self.point_out.value_out().map(|pt| *pt).unwrap_or_default()
    }
    pub fn points_out(&self) -> impl Iterator<Item = &piet::kurbo::Point> {
        self.point_out.values_out().iter()
    }
}

impl super::Node for PointNode {}
impl super::NodeInfo for PointNode {
    fn inputs() -> usize {
        2
    }

    fn outputs() -> usize {
        1
    }

    fn title() -> String {
        "Point".to_string()
    }
}

impl super::NodeDowncast for PointNode {
    fn try_downcast(from: &super::Nodes) -> Option<&Self> {
        todo!()
    }

    fn try_downcast_mut(from: &mut super::Nodes) -> Option<&mut Self> {
        match from {
            super::Nodes::Point(node) => Some(node),
            _ => None,
        }
    }
}

impl super::InputNode<super::Nodes> for PointNode {
    fn show_input(
        pin: &egui_snarl::InPin,
        ui: &mut egui::Ui,
        scale: f32,
        snarl: &mut egui_snarl::Snarl<super::Nodes>,
    ) -> egui_snarl::ui::PinInfo {
        match pin.id.input {
            0 => super::show_input_for_number_pin("X", pin, ui, scale, snarl, |id, snarl| {
                &mut super::get_node_mut::<Self>(snarl, id.node).x_in
            }),
            1 => super::show_input_for_number_pin("Y", pin, ui, scale, snarl, |id, snarl| {
                &mut super::get_node_mut::<Self>(snarl, id.node).y_in
            }),
            _ => unreachable!(),
        }
    }
}

impl super::OutputNode<super::Nodes> for PointNode {
    fn show_output(
        pin: &egui_snarl::OutPin,
        ui: &mut egui::Ui,
        scale: f32,
        snarl: &mut egui_snarl::Snarl<super::Nodes>,
    ) -> egui_snarl::ui::PinInfo {
        ui.label("Point");
        PinInfo::circle().with_fill(crate::POINT_COLOR)
    }
}
