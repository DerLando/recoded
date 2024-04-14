use eframe::App;
use eframe::CreationContext;
use egui::Color32;
use egui::Style;
use egui_snarl::ui::SnarlStyle;
use egui_snarl::ui::SnarlViewer;
use egui_snarl::Snarl;
use nodes::InputNode;
use nodes::OutputNode;

mod nodes;

const NUMBER_COLOR: egui::Color32 = egui::Color32::from_rgb(255, 255, 0);
const UNCONNECTED_COLOR: egui::Color32 = egui::Color32::from_rgb(50, 50, 50);

struct NodeGraphViewer;

impl SnarlViewer<nodes::Nodes> for NodeGraphViewer {
    fn title(&mut self, node: &nodes::Nodes) -> String {
        node.title()
    }

    fn outputs(&mut self, node: &nodes::Nodes) -> usize {
        node.outputs()
    }

    fn inputs(&mut self, node: &nodes::Nodes) -> usize {
        node.inputs()
    }

    fn show_input(
        &mut self,
        pin: &egui_snarl::InPin,
        ui: &mut egui::Ui,
        scale: f32,
        snarl: &mut egui_snarl::Snarl<nodes::Nodes>,
    ) -> egui_snarl::ui::PinInfo {
        match &mut snarl[pin.id.node] {
            nodes::Nodes::ConstantValueNode(_) => unreachable!(),
            nodes::Nodes::Sink(_) => nodes::sink::show_input(pin, ui, scale, &snarl),
            nodes::Nodes::Range(node) => nodes::range::RangeNode::show_input(pin, ui, scale, snarl), // TODO: How can we hand down snarl properly here? since the node needs to be mut, we must also mutably borrow the snarl to get that reference, so I guess the signature of the InputNode trait should include a mutable reference there, too
        }
    }

    fn show_output(
        &mut self,
        pin: &egui_snarl::OutPin,
        ui: &mut egui::Ui,
        scale: f32,
        snarl: &mut egui_snarl::Snarl<nodes::Nodes>,
    ) -> egui_snarl::ui::PinInfo {
        match &mut snarl[pin.id.node] {
            nodes::Nodes::ConstantValueNode(ref mut node) => node.show_output(ui),
            nodes::Nodes::Sink(_) => unreachable!(),
            nodes::Nodes::Range(_) => nodes::range::RangeNode::show_output(pin, ui, scale, snarl),
        }
    }

    fn input_color(
        &mut self,
        pin: &egui_snarl::InPin,
        style: &Style,
        snarl: &mut egui_snarl::Snarl<nodes::Nodes>,
    ) -> Color32 {
        NUMBER_COLOR
    }

    fn output_color(
        &mut self,
        pin: &egui_snarl::OutPin,
        style: &Style,
        snarl: &mut egui_snarl::Snarl<nodes::Nodes>,
    ) -> Color32 {
        NUMBER_COLOR
    }

    fn graph_menu(
        &mut self,
        pos: egui::Pos2,
        ui: &mut egui::Ui,
        _scale: f32,
        snarl: &mut Snarl<nodes::Nodes>,
    ) {
        ui.label("Add node");
        if ui.button("Constant").clicked() {
            snarl.insert_node(pos, nodes::Nodes::ConstantValueNode(nodes::constant_value::ConstantValueNode::default()));
            ui.close_menu();
        }
        if ui.button("Sink").clicked() {
            snarl.insert_node(pos, nodes::Nodes::Sink(nodes::sink::SinkNode));
            ui.close_menu();
        }
        if ui.button("Range").clicked() {
            snarl.insert_node(pos, nodes::Nodes::Range(nodes::range::RangeNode::default()));
            ui.close_menu();
        }
    }
}

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

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };

    eframe::run_native(
        "recoded",
        native_options,
        Box::new(|cx| Box::new(NodeGraphApp::new(cx))),
    )

    // println!("Hello, world!");
    // let mut rc = piet_svg::RenderContext::new(piet::kurbo::Size {
    //     width: 200.0,
    //     height: 200.0,
    // });
    // let inputs = std::fs::read_to_string("inputs.json")?;
    // let inputs = serde_json::from_str(&inputs)?;
    // draw(&mut rc, &inputs).expect("Succeeded in drawing");
    // let file = std::fs::File::create("test.svg").expect("OK");

    // rc.write(file);

    // Ok(())
}

fn draw(rc: &mut impl piet::RenderContext, input: &GraphInputs) -> Result<(), piet::Error> {
    rc.clear(None, piet::Color::WHITE);
    rc.transform(piet::kurbo::Affine::translate(piet::kurbo::Vec2::new(
        100.0, 100.0,
    )));
    for shape in generate_shapes(input) {
        rc.stroke(shape, &piet::Color::BLACK, 1.0);
    }

    Ok(())
}

enum NaturalConstant {
    Pi,
    Tau,
    /// Golden ratio
    Phi,
    /// Golden angle
    Rho,
}

impl NaturalConstant {
    fn value(&self) -> f64 {
        match self {
            NaturalConstant::Pi => std::f64::consts::PI,
            NaturalConstant::Tau => std::f64::consts::TAU,
            NaturalConstant::Phi => (1.0 + 5.0_f64.sqrt()) / 2.0,
            NaturalConstant::Rho => {
                NaturalConstant::Tau.value()
                    - NaturalConstant::Tau.value() / NaturalConstant::Phi.value()
            }
        }
    }
}

#[derive(serde::Deserialize)]
struct GraphInputs {
    circle_count: usize,
    shapes_radius_step: f64,
    shapes_angle_step: f64,
    circle_radius: f64,
}

fn generate_shapes(input: &GraphInputs) -> Vec<impl piet::kurbo::Shape> {
    let mut shapes = Vec::with_capacity(input.circle_count);
    for i in 0..input.circle_count {
        let angle = NaturalConstant::Rho.value() * input.shapes_angle_step * i as f64;
        let radius = input.shapes_radius_step * i as f64 * 3.0;
        let center = PointPolar::new(radius, angle);
        let shape = piet::kurbo::Circle::new(center, input.circle_radius);
        shapes.push(shape);
    }

    shapes
}

struct PointPolar {
    radius: f64,
    angle: f64,
}

impl PointPolar {
    const fn new(radius: f64, angle: f64) -> Self {
        Self { radius, angle }
    }

    fn to_carthesian(&self) -> piet::kurbo::Point {
        let x = self.radius * self.angle.cos();
        let y = self.radius * self.angle.sin();
        piet::kurbo::Point::new(x, y)
    }
}

impl Into<piet::kurbo::Point> for PointPolar {
    fn into(self) -> piet::kurbo::Point {
        self.to_carthesian()
    }
}
