use egui_snarl::ui::PinInfo;

use crate::{
    pins::{IPin, InputPin, InputPinId, OPin, OutputPin},
    values::Values,
};

use super::NodeInfo;

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct PointNode {
    x_in: InputPin<f64>,
    y_in: InputPin<f64>,
    point_out: OutputPin<piet::kurbo::Point>,
}

impl PointNode {
    pub(crate) fn recalc(&mut self) {
        let x = self.x_in.values_out();
        let y = self.y_in.values_out();
        let pts = x
            .iter()
            .zip(y.iter())
            .map(|(x, y)| piet::kurbo::Point::new(*x, *y));
        self.point_out.values_in(pts.clone());
        println!("Solved points: {:?}", pts);
    }
    pub fn point_out(&self) -> piet::kurbo::Point {
        self.point_out.value_out().map(|pt| *pt).unwrap_or_default()
    }
    pub fn points_out(&self) -> impl Iterator<Item = &piet::kurbo::Point> {
        self.point_out.values_out().iter()
    }

    fn values_in_inner(pin: &mut InputPin<f64>, values: &crate::values::Values) {
        match values {
            crate::values::Values::Int(values) => pin.values_in(values.iter().map(|v| *v as f64)),
            crate::values::Values::Float(values) => pin.values_in(values.iter().map(|v| *v)),
            _ => unreachable!(),
        }
    }
}

impl super::Node for PointNode {}
impl NodeInfo for PointNode {
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

    fn values_in(&mut self, id: InputPinId, values: &Values) {
        if id.0 >= Self::inputs() {
            return;
        }

        match id.0 {
            0 => Self::values_in_inner(&mut self.x_in, values),
            1 => Self::values_in_inner(&mut self.y_in, values),
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

    fn values_out(&self, id: crate::pins::OutputPinId) -> Values {
        Values::Point(self.point_out.values_out().clone())
    }
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct PointPolarNode {
    center_in: InputPin<piet::kurbo::Point>,
    radius_in: InputPin<f64>,
    angle_in: InputPin<f64>,
    point_out: OutputPin<piet::kurbo::Point>,
}

impl PointPolarNode {
    fn to_carthesian(center: &piet::kurbo::Point, radius: f64, angle: f64) -> piet::kurbo::Point {
        *center + piet::kurbo::Vec2::new(angle.cos() * radius, angle.sin() * radius)
    }
    pub fn recalc(&mut self) {
        self.point_out.values_in(
            self.center_in
                .values_out()
                .iter()
                .zip(
                    self.radius_in
                        .values_out()
                        .iter()
                        .zip(self.angle_in.values_out()),
                )
                .map(|(center, (radius, angle))| Self::to_carthesian(center, *radius, *angle)),
        );
    }
}

impl super::Node for PointPolarNode {}
impl super::NodeInfo for PointPolarNode {
    fn inputs() -> usize {
        3
    }

    fn outputs() -> usize {
        1
    }

    fn title() -> String {
        "PointPolat".to_string()
    }
}

impl super::NodeDowncast for PointPolarNode {
    fn try_downcast(from: &super::Nodes) -> Option<&Self> {
        todo!()
    }

    fn try_downcast_mut(from: &mut super::Nodes) -> Option<&mut Self> {
        match from {
            super::Nodes::PointPolar(node) => Some(node),
            _ => None,
        }
    }
}

impl super::InputNode<super::Nodes> for PointPolarNode {
    fn show_input(
        pin: &egui_snarl::InPin,
        ui: &mut egui::Ui,
        scale: f32,
        snarl: &mut egui_snarl::Snarl<super::Nodes>,
    ) -> PinInfo {
        match pin.id.input {
            0 => super::show_point_input("Center", pin, ui, scale, snarl, |id, snarl| {
                &mut super::get_node_mut::<Self>(snarl, id.node).center_in
            }),
            1 => super::show_input_for_number_pin("Radius", pin, ui, scale, snarl, |id, snarl| {
                &mut super::get_node_mut::<Self>(snarl, id.node).radius_in
            }),
            2 => super::show_input_for_number_pin("Angle", pin, ui, scale, snarl, |id, snarl| {
                &mut super::get_node_mut::<Self>(snarl, id.node).angle_in
            }),
            _ => unreachable!(),
        }
    }

    fn values_in(&mut self, id: InputPinId, values: &Values) {
        match id.0 {
            0 => match values {
                Values::Point(values) => self.center_in.values_in(values.iter().cloned()),
                _ => (),
            },
            1 => match values {
                Values::Int(values) => self.radius_in.values_in(values.iter().cloned()),
                Values::Float(values) => self.radius_in.values_in(values.iter().cloned()),
                _ => (),
            },
            2 => match values {
                Values::Int(values) => self.angle_in.values_in(values.iter().cloned()),
                Values::Float(values) => self.angle_in.values_in(values.iter().cloned()),
                _ => (),
            },
            _ => unreachable!(),
        }
    }
}

impl super::OutputNode<super::Nodes> for PointPolarNode {
    fn show_output(
        pin: &egui_snarl::OutPin,
        ui: &mut egui::Ui,
        scale: f32,
        snarl: &mut egui_snarl::Snarl<super::Nodes>,
    ) -> PinInfo {
        ui.label("Point");
        PinInfo::circle().with_fill(crate::POINT_COLOR)
    }

    fn values_out(&self, id: crate::pins::OutputPinId) -> Values {
        Values::Point(self.point_out.values_out().clone())
    }
}
