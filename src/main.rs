use app::NodeGraphApp;
use eframe::App;
use eframe::CreationContext;
use egui::Color32;
use egui::Style;
use egui_snarl::ui::SnarlStyle;
use egui_snarl::ui::SnarlViewer;
use egui_snarl::Snarl;
use nodes::InputNode;
use nodes::OutputNode;

mod app;
mod nodes;
mod pins;
mod shapes;
mod solver;
mod viewer;

const NUMBER_COLOR: egui::Color32 = egui::Color32::from_rgb(255, 255, 0);
const POINT_COLOR: egui::Color32 = egui::Color32::from_rgb(0, 255, 255);
const SHAPE_COLOR: egui::Color32 = egui::Color32::from_rgb(255, 0, 255);
const UNCONNECTED_COLOR: egui::Color32 = egui::Color32::from_rgb(50, 50, 50);

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
