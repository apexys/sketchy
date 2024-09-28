mod sketch_renderer;

use eframe::{egui, CreationContext};
use eframe::egui::PaintCallback;
use eframe::egui_wgpu;
use sketch_renderer::SketchRenderer;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions{
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 400.0]),
        ..Default::default()
    };
    eframe::run_native("Sketchy", options, Box::new(|cc| {
        Ok(Box::new(SketchyApp::new(cc)))
    }))
}

struct SketchyApp {
    renderer: SketchRenderer
}

impl SketchyApp {
    pub fn new(cc: &CreationContext) -> Self{
        Self{
            renderer: SketchRenderer::new(cc)
        }
    }
}

impl eframe::App for SketchyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.renderer.custom_painting(ui);



        });
    }
}
