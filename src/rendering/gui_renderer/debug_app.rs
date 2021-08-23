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
        let elapsed_str = format!("Time: {:.2}", self.start_time.elapsed().as_secs_f64());

        egui::SidePanel::left("Debug Info")
            .resizable(true)
            .show(ctx, |ui| {
                ui.add(Label::new(elapsed_str).text_style(TextStyle::Heading))
            }
        );
    }
}