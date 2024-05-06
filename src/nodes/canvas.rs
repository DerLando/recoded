use piet::RenderContext;

use crate::{
    pins::{IPin, InputPin, OPin},
    shapes::Shapes,
    values::Values,
};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CanvasNode {
    width: InputPin<f64>,
    height: InputPin<f64>,
    shapes: InputPin<Shapes>,
    image_buffer: Vec<u8>,
    image_changed: bool,
}

impl CanvasNode {
    fn width(&self) -> f64 {
        self.width.value_out().cloned().unwrap_or(400.0)
    }

    fn height(&self) -> f64 {
        self.height.value_out().cloned().unwrap_or(300.0)
    }

    pub fn recalc(&mut self) {
        self.image_buffer = self.draw();
        self.image_changed = true;
    }

    fn draw(&self) -> Vec<u8> {
        let mut rc =
            piet_svg::RenderContext::new(piet::kurbo::Size::new(self.width(), self.height()));
        rc.clear(None, piet::Color::WHITE);
        rc.save();
        // TODO: Why is clipping in percent? I don't get it...
        rc.clip(piet::kurbo::Rect::new(0.0, 0.0, 100.0, 100.0));
        for shape in self.shapes.values_out() {
            rc.stroke(shape.get_shape(), &piet::Color::BLACK, 1.0);
        }
        let mut buffer: Vec<u8> = Vec::new();
        rc.write(&mut buffer).expect("Write worked");
        println!("Drew new image");
        buffer
    }
}

impl Default for CanvasNode {
    fn default() -> Self {
        Self {
            width: InputPin::default(),
            height: InputPin::default(),
            shapes: InputPin::default(),
            image_buffer: Vec::new(),
            image_changed: false,
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
            0 => super::show_input_for_number_pin("Width", pin, ui, scale, snarl, |id, snarl| {
                &mut super::get_node_mut::<Self>(snarl, id.node).width
            }),
            1 => super::show_input_for_number_pin("Height", pin, ui, scale, snarl, |id, snarl| {
                &mut super::get_node_mut::<Self>(snarl, id.node).height
            }),
            2 => super::show_shape_input("Shapes", pin, ui, scale, snarl, |id, snarl| {
                &mut super::get_node_mut::<Self>(snarl, id.node).shapes
            }),
            _ => unreachable!(),
        }
    }

    fn values_in(&mut self, id: crate::pins::InputPinId, values: &crate::values::Values) {
        match id.0 {
            0 => match values {
                Values::Int(values) => self
                    .width
                    .value_in(values.iter().next().cloned().unwrap_or(400)),
                Values::Float(values) => self
                    .width
                    .value_in(values.iter().next().cloned().unwrap_or(400.0)),
                _ => (),
            },
            1 => match values {
                Values::Int(values) => self
                    .height
                    .value_in(values.iter().next().cloned().unwrap_or(300)),
                Values::Float(values) => self
                    .height
                    .value_in(values.iter().next().cloned().unwrap_or(300.0)),
                _ => (),
            },
            2 => match values {
                Values::Shape(values) => self.shapes.values_in(values.iter().cloned()),
                Values::Circle(values) => self
                    .shapes
                    .values_in(values.iter().map(|c| Shapes::Circle(c.clone()))),
                _ => (),
            },
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
        let changed = super::get_node_mut::<Self>(snarl, pin.id.node).image_changed;
        let uri = format!("bytes://canvas{}.svg", pin.id.node.0);
        let width = super::get_node_mut::<Self>(snarl, pin.id.node).width();
        let height = super::get_node_mut::<Self>(snarl, pin.id.node).height();
        if changed {
            ui.ctx().forget_image(&uri);
        }
        let image = {
            if changed {
                egui::Image::from_bytes(
                    uri.to_owned(),
                    super::get_node_mut::<Self>(snarl, pin.id.node)
                        .image_buffer
                        .clone(),
                )
                .show_loading_spinner(true)
            } else {
                egui::Image::from_uri(uri)
            }
        };
        // TODO: Image is scaled weirdly, I don't get why :/
        // At least clipping works properly-ish now...
        ui.add(image.max_size(egui::Vec2::new(width as f32 * scale, height as f32 * scale)));

        if changed {
            super::get_node_mut::<Self>(snarl, pin.id.node).image_changed = false;
        }
        egui_snarl::ui::PinInfo::triangle()
    }

    fn values_out(&self, id: crate::pins::OutputPinId) -> crate::values::Values {
        todo!()
    }
}
