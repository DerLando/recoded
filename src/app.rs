use eframe::{App, CreationContext};
use egui_snarl::{ui::SnarlStyle, Snarl};

use crate::{nodes, viewer::NodeGraphViewer};
// TODO: I also need a generic solver which can solve a [`Snarl`]
// without showing any ui. This will allow hosting graphs in an axum
// server and sending json serialized inputs and getting the result back
pub struct NodeGraphApp {
    snarl: Snarl<nodes::Nodes>,
    style: SnarlStyle,
}

impl NodeGraphApp {
    pub fn new(cx: &CreationContext) -> Self {
        let snarl = match cx.storage {
            None => Snarl::new(),
            Some(storage) => {
                let snarl = storage
                    .get_string("snarl")
                    .and_then(|snarl| serde_json::from_str(&snarl).ok())
                    .unwrap_or_else(Snarl::new);
                snarl
            }
        };

        let style = match cx.storage {
            None => SnarlStyle::new(),
            Some(storage) => {
                let style = storage
                    .get_string("style")
                    .and_then(|style| serde_json::from_str(&style).ok())
                    .unwrap_or_else(SnarlStyle::new);
                style
            }
        };

        NodeGraphApp { snarl, style }
    }
}

impl App for NodeGraphApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui_extras::install_image_loaders(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            self.snarl.show(
                &mut NodeGraphViewer,
                &self.style,
                egui::Id::new("snarl"),
                ui,
            );
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        let snarl = serde_json::to_string(&self.snarl).unwrap();
        storage.set_string("snarl", snarl);

        let style = serde_json::to_string(&self.style).unwrap();
        storage.set_string("style", style);
    }
}
