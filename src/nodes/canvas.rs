use piet::RenderContext;

use crate::shapes::Shapes;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CanvasNode {
    width: f64,
    height: f64,
    shapes: Vec<Shapes>,
}

impl CanvasNode {
    fn draw(&self) -> Vec<u8> {
        let mut rc = piet_svg::RenderContext::new(piet::kurbo::Size::new(self.width, self.height));
        rc.clear(None, piet::Color::WHITE);
        for shape in &self.shapes {
            rc.stroke(shape.get_shape(), &piet::Color::BLACK, 1.0);
        }
        let mut buffer: Vec<u8> = Vec::new();
        rc.write(&mut buffer).expect("Write worked");
        buffer
    }
}

impl Default for CanvasNode {
    fn default() -> Self {
        Self {
            width: 400.0,
            height: 300.0,
            shapes: vec![Shapes::Circle(piet::kurbo::Circle::default())],
        }
    }
}

impl super::Node for CanvasNode {}
impl super::NodeInfo for CanvasNode {
    fn inputs() -> usize {
        3
    }

    fn outputs() -> usize {
        1
    }

    fn title() -> String {
        "Canvas".to_string()
    }
}
impl super::NodeDowncast for CanvasNode {
    fn try_downcast(from: &super::Nodes) -> Option<&Self> {
        todo!()
    }

    fn try_downcast_mut(from: &mut super::Nodes) -> Option<&mut Self> {
        match from {
            super::Nodes::Canvas(node) => Some(node),
            _ => None,
        }
    }
}
impl super::InputNode<super::Nodes> for CanvasNode {
    fn show_input(
        pin: &egui_snarl::InPin,
        ui: &mut egui::Ui,
        scale: f32,
        snarl: &mut egui_snarl::Snarl<super::Nodes>,
    ) -> egui_snarl::ui::PinInfo {
        match pin.id.input {
            0 => super::show_number_input("Width", pin, ui, scale, snarl, |id, snarl| {
                &mut super::get_node_mut::<Self>(snarl, id.node).width
            }),
            1 => super::show_number_input("Height", pin, ui, scale, snarl, |id, snarl| {
                &mut super::get_node_mut::<Self>(snarl, id.node).height
            }),
            // TODO: This input should take the whole list of shapes!
            2 => super::show_shape_input("Shapes", pin, ui, scale, snarl, |id, snarl| {
                &mut super::get_node_mut::<Self>(snarl, id.node).shapes[0]
            }),
            _ => unreachable!(),
        }
    }
}
impl super::OutputNode<super::Nodes> for CanvasNode {
    fn show_output(
        pin: &egui_snarl::OutPin,
        ui: &mut egui::Ui,
        scale: f32,
        snarl: &mut egui_snarl::Snarl<super::Nodes>,
    ) -> egui_snarl::ui::PinInfo {
        let uri = format!("bytes://canvas{}.svg", pin.id.node.0);
        let image = egui::Image::from_bytes(
            uri.to_owned(),
            super::get_node_mut::<Self>(snarl, pin.id.node).draw(),
        )
        .max_width(200.0 * scale)
        .shrink_to_fit()
        .show_loading_spinner(true);
        ui.add(image);

        // TODO: Bad for performance, but necessary to update the image. Would be better to somehow cache the image in the struct itself, so all inputs can refresh it on change
        ui.ctx().forget_image(&uri);
        egui_snarl::ui::PinInfo::triangle()
    }
}
