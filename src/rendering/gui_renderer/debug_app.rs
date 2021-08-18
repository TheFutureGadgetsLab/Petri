use egui::*;
use std::time::Instant;


#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct DebugApp {
    start_time: Instant,
}

impl Default for DebugApp {
    fn default() -> Self {
        Self {
            start_time: Instant::now(),
        }
    }
}

impl epi::App for DebugApp {
    fn name(&self) -> &str {
        "🎨 Color test"
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        egui::Window::new("Debug Info")
            .resizable(true)
            .anchor(Align2::LEFT_TOP, [0., 0.])
            .show(ctx, |ui| {
                ui.label(format!("time elapsed: {:.2}", self.start_time.elapsed().as_secs_f64()));
            }
        );
    }
}