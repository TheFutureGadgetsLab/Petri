use egui::*;
use crate::simulation::{Simulation, Time};


#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct DebugApp;

impl DebugApp {
    pub fn update(&mut self, ctx: &egui::CtxRef, simulation: &Simulation) {
        let time = simulation.resources.get_mut::<Time>().unwrap();

        let draw_str = format!(
            "Time: {:.2} | FPS: {}", 
            time.time_since_start().as_secs_f64(),
            time.tick_rate
        );
        egui::TopBottomPanel::top("Debug Info")
            .show(ctx, |ui| {
                ui.add(Label::new(draw_str).text_style(TextStyle::Heading));
            }
        );
    }
}