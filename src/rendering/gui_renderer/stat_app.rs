use egui;

use crate::{
    config::Config,
    rendering::Display,
    simulation::{Simulation, Time},
};

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct StatApp;

impl StatApp {
    pub fn update(&mut self, ctx: &egui::CtxRef, display: &Display, simulation: &Simulation) {
        let time = simulation.resources.get::<Time>().unwrap();
        let config = simulation.resources.get::<Config>().unwrap();
        let cam = &display.cam;

        egui::SidePanel::left("Debug Info").show(ctx, |ui| {
            ui.heading("Stats");
            ui.separator();
            egui::Grid::new("Stats Grid")
                .striped(true)
                .num_columns(2)
                .show(ui, |ui| {
                    ui.label("Time:");
                    ui.label(format!("{:.2}", time.time_since_start().as_secs_f32()));
                    ui.end_row();

                    ui.label("Ticks / s:");
                    ui.label(format!("{:}", time.tick_rate));
                    ui.end_row();

                    ui.label("Entities:");
                    ui.label(format!("{:}", simulation.world.len()));
                    ui.end_row();
                });

            ui.heading("Camera");
            ui.separator();
            egui::Grid::new("Camera Grid")
                .striped(true)
                .num_columns(2)
                .show(ui, |ui| {
                    // Camera
                    ui.label("Position:");
                    ui.label(format!("({:.2}, {:.2})", cam.pos().x, cam.pos().y));
                    ui.end_row();

                    ui.label("Window Size:");
                    ui.label(format!("({:.2}, {:.2})", cam.window_size.x, cam.window_size.y));
                    ui.end_row();

                    ui.label("Zoom Factor");
                    ui.label(format!("({:.2})", cam.zoom));
                    ui.end_row();

                    ui.label("Mouse Position:");
                    ui.label(format!(
                        "({:2}, {:2})",
                        cam.screen2world(display.mouse.pos).x,
                        cam.screen2world(display.mouse.pos).y
                    ));
                    ui.end_row();
                });

            ui.heading("Config");
            ui.separator();
            egui::Grid::new("Config Grid")
                .striped(true)
                .num_columns(2)
                .show(ui, |ui| {
                    ui.label("Bounds:");
                    ui.label(format!(
                        "({:}, {:}) | ({:}, {:})",
                        config.bounds.0.x, config.bounds.0.y, config.bounds.1.x, config.bounds.1.y
                    ));
                    ui.end_row();
                });
        });
    }
}
