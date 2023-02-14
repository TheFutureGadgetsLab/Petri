use egui;
use num_format::{Locale, ToFormattedString};

use crate::{
    config::Config,
    rendering::Display,
    simulation::{Simulation, Ticker},
};

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct StatApp;

impl StatApp {
    pub fn update(&mut self, ui: &mut egui::Ui, display: &Display, simulation: &Simulation) {
        let time = simulation.world.get_resource::<Ticker>().unwrap();
        let config = simulation.get_config();
        let cam = &display.cam;

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
                ui.label(simulation.world.entities().len().to_formatted_string(&Locale::en));
                ui.end_row();
            });

        ui.heading("Camera");
        ui.separator();
        egui::Grid::new("Camera Grid")
            .striped(true)
            .num_columns(2)
            .show(ui, |ui| {
                ui.label("World X Bounds:");
                ui.label(format!("({:.2}, {:.2})", cam.world_xbounds.x, cam.world_xbounds.y));
                ui.end_row();

                ui.label("World Y Bounds:");
                ui.label(format!("({:.2}, {:.2})", cam.world_ybounds.x, cam.world_ybounds.y));
                ui.end_row();

                ui.label("Screen width:");
                ui.label(format!("{:}", cam.screen_width));
                ui.end_row();

                ui.label("Screen height:");
                ui.label(format!("{:}", cam.screen_height));
                ui.end_row();
            });

        self.show_config(ui, config);
    }

    fn show_config(&self, ui: &mut egui::Ui, config: &Config) {
        ui.heading("Config");
        ui.separator();
        let grid = egui::Grid::new("Config Grid").striped(true).num_columns(2);
        grid.show(ui, |ui| {
            ui.label("N Cells:");
            ui.label(config.n_cells.to_formatted_string(&Locale::en));
            ui.end_row();

            ui.label("Cell Radius:");
            ui.label(format!("{:.2}", config.cell_radius));
            ui.end_row();

            ui.label("Bounds:");
            ui.label(format!(
                "({:}, {:}) | ({:}, {:})",
                config.bounds.0.x, config.bounds.0.y, config.bounds.1.x, config.bounds.1.y
            ));
            ui.end_row();

            ui.label("Max Render FPS:");
            ui.label(format!("{:}", config.max_render_fps));
            ui.end_row();

            ui.label("Max Sim TPS:");
            ui.label(format!("{:}", config.max_sim_tps));
            ui.end_row();
        });
    }
}
