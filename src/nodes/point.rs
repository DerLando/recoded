use egui_snarl::ui::PinInfo;

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct PointNode {
    point: piet::kurbo::Point,
}

impl PointNode {
    pub fn point_out(&self) -> piet::kurbo::Point {
        self.point
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
            0 => super::show_number_input("X", pin, ui, scale, snarl, |id, snarl| {
                &mut super::get_node_mut::<Self>(snarl, id.node).point.x
            }),
            1 => super::show_number_input("Y", pin, ui, scale, snarl, |id, snarl| {
                &mut super::get_node_mut::<Self>(snarl, id.node).point.y
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
