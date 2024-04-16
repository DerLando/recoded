use egui::{Color32, Style};
use egui_snarl::{ui::SnarlViewer, Snarl};

use crate::nodes::{self, InputNode, OutputNode};

pub(super) struct NodeGraphViewer;

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
            nodes::Nodes::Range(_) => nodes::range::RangeNode::show_input(pin, ui, scale, snarl),
            nodes::Nodes::Point(_) => nodes::point::PointNode::show_input(pin, ui, scale, snarl),
            nodes::Nodes::Circle(_) => nodes::circle::CircleNode::show_input(pin, ui, scale, snarl),
            nodes::Nodes::Canvas(_) => nodes::canvas::CanvasNode::show_input(pin, ui, scale, snarl),
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
            nodes::Nodes::Point(_) => nodes::point::PointNode::show_output(pin, ui, scale, snarl),
            nodes::Nodes::Circle(_) => {
                nodes::circle::CircleNode::show_output(pin, ui, scale, snarl)
            }
            nodes::Nodes::Canvas(_) => {
                nodes::canvas::CanvasNode::show_output(pin, ui, scale, snarl)
            }
        }
    }

    fn input_color(
        &mut self,
        pin: &egui_snarl::InPin,
        style: &Style,
        snarl: &mut egui_snarl::Snarl<nodes::Nodes>,
    ) -> Color32 {
        crate::NUMBER_COLOR
    }

    fn output_color(
        &mut self,
        pin: &egui_snarl::OutPin,
        style: &Style,
        snarl: &mut egui_snarl::Snarl<nodes::Nodes>,
    ) -> Color32 {
        crate::NUMBER_COLOR
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
        if ui.button("Point").clicked() {
            snarl.insert_node(pos, nodes::Nodes::Point(nodes::point::PointNode::default()));
            ui.close_menu();
        }
        if ui.button("Circle").clicked() {
            snarl.insert_node(
                pos,
                nodes::Nodes::Circle(nodes::circle::CircleNode::default()),
            );
            ui.close_menu();
        }
        if ui.button("Canvas").clicked() {
            snarl.insert_node(
                pos,
                nodes::Nodes::Canvas(nodes::canvas::CanvasNode::default()),
            );
            ui.close_menu();
        }
    }
}
