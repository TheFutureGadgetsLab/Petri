use egui::*;
use crate::driver::Stats;


#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct DebugApp;

impl DebugApp {
    pub fn update(&mut self, ctx: &egui::CtxRef, stats: &Stats) {
        let elapsed_str = format!("Time: {:.2}", stats.secs_since_start());
        let fps_str = format!("FPS: {}", stats.fps);

        egui::SidePanel::left("Debug Info")
            .resizable(true)
            .show(ctx, |ui| {
                ui.add(Label::new(elapsed_str).text_style(TextStyle::Heading));
                ui.add(Label::new(fps_str).text_style(TextStyle::Heading));
            }
        );
    }
}