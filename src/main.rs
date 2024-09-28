mod sketch_renderer;

use eframe::egui;
use eframe::egui::PaintCallback;
use eframe::egui_wgpu;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions{
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 400.0]),
        ..Default::default()
    };
    eframe::run_native("Sketchy", options, Box::new(|cc| {
        Ok(Box::<SketchyApp>::default())
    }))
}

#[derive(Default)]
struct SketchyApp {}

impl SketchyApp {}

impl eframe::App for SketchyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("test");



        });
    }
}
