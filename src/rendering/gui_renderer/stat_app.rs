use egui::*;
use crate::simulation::{Simulation, Time};


#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct DebugApp;

impl DebugApp {
    pub fn update(&mut self, ctx: &egui::CtxRef, simulation: &Simulation) {
        let time = simulation.resources.get_mut::<Time>().unwrap();

        let time_str = format!(
            "Time: {:.2}", 
            time.time_since_start().as_secs_f64(),
        );
        let tick_str = format!(
            "Ticks / s: {}", 
            time.tick_rate
        );

        egui::TopBottomPanel::top("Debug Info")
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.add(Label::new(time_str).text_style(TextStyle::Heading));
                    ui.separator();
                    ui.add(Label::new(tick_str).text_style(TextStyle::Heading));
                });
            });
    }
}