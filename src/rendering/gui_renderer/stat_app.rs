use crate::{
    rendering::{Camera, Display},
    simulation::{Simulation, Time},
};

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct StatApp;

impl StatApp {
    pub fn update(&mut self, ctx: &egui::CtxRef, display: &Display, simulation: &Simulation) {
        let time = simulation.resources.get::<Time>().unwrap();
        let cam = simulation.resources.get::<Camera>().unwrap();

        egui::SidePanel::left("Debug Info").show(ctx, |ui| {
            ui.style_mut().wrap = Some(false);
            ui.heading("Stats");
            ui.label(format!("Time: {:.2}", time.time_since_start().as_secs_f32()));
            ui.label(format!("Ticks / s: {:}", time.tick_rate));
            ui.label(format!("Entities: {}", simulation.world.len()));

            ui.separator();

            ui.heading("Camera");
            ui.label(format!(
                "Translation: ({:.2}, {:.2})",
                cam.translation.x, cam.translation.y
            ));
            ui.label(format!(
                "Window Size: ({:.2}, {:.2})",
                cam.window_size.x, cam.window_size.y
            ));
            ui.label(format!("Zoom Factor: ({:.2})", cam.zoom));
            ui.label(format!("Mouse Position: ({})", cam.screen2world(display.mouse.pos)));
        });
    }
}
