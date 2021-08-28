use crate::simulation::{Simulation, Time};
use crate::rendering::Camera;
use glam::Vec2;

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct StatApp;

impl StatApp {
    pub fn update(&mut self, ctx: &egui::CtxRef, simulation: &Simulation) {
        let time = simulation.resources.get_mut::<Time>().unwrap();
        let cam = simulation.resources.get_mut::<Camera>().unwrap();

        let cam_pos = cam.transform.translation;
        let cam_scale = Vec2::new(
            cam.transform.matrix2.row(0).x,
            cam.transform.matrix2.row(1).y,
        );

        egui::SidePanel::left("Debug Info")
            .show(ctx, |ui| {
                ui.heading("Stats");
                ui.label(format!("Time: {:.2}", time.time_since_start().as_secs_f32()));
                ui.label(format!("Ticks / s: {:}", time.tick_rate));

                ui.separator();

                ui.heading("Camera");
                ui.label(format!("Pos:   ({:.5}, {:.5})", cam_pos.x, cam_pos.y));
                ui.label(format!("Scale: ({:.5}, {:.5})", cam_scale.x, cam_scale.y));
                ui.label(format!("Size:  ({:.5}, {:.5})", cam.window_size.x, cam.window_size.y));
            });
    }
}