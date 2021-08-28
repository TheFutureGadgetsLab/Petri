use crate::simulation::{Simulation, Time};
use crate::rendering::Camera;

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct StatApp;

impl StatApp {
    pub fn update(&mut self, ctx: &egui::CtxRef, simulation: &Simulation) {
        let time = simulation.resources.get_mut::<Time>().unwrap();
        let cam = simulation.resources.get_mut::<Camera>().unwrap();

        egui::SidePanel::left("Debug Info")
            .show(ctx, |ui| {
                ui.heading("Stats");
                ui.label(format!("Time: {:.2}", time.time_since_start().as_secs_f32()));
                ui.label(format!("Ticks / s: {:}", time.tick_rate));

                ui.separator();

                ui.heading("Camera");
                ui.label(format!("Pos:   {:.2}", cam.pos));
                ui.label(format!("Scale: {:.2}", cam.scale));
                ui.label(format!("Size:  {:.2}", cam.size));
            });
    }
}