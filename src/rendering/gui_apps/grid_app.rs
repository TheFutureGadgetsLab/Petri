use egui::{
    epaint::Color32,
    plot::{Line, Plot, PlotPoints, PlotUi},
};
use ultraviolet::Vec2;

use crate::{
    config::Config,
    rendering::Display,
    simulation::{DenseGrid, Simulation},
};

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct GridApp {
    pub visible: bool,
}

impl Default for GridApp {
    fn default() -> Self {
        Self { visible: true }
    }
}

impl GridApp {
    pub fn update(&mut self, ui: &mut egui::Ui, display: &Display, simulation: &Simulation) {
        let base_plot = self.build_base_plot(display);
        base_plot.show(ui, |plot_ui| {
            self.show_sim_grid(plot_ui, simulation);
            self.show_spatial_grid(plot_ui, simulation);
        });
    }

    pub fn show_sim_grid(&self, plot_ui: &mut PlotUi, simulation: &Simulation) {
        let config = simulation.world.get_resource::<Config>().unwrap();
        let (minx, miny) = (config.bounds.0.x as f64, config.bounds.0.y as f64);
        let (maxx, maxy) = (config.bounds.1.x as f64, config.bounds.1.y as f64);

        let path = vec![[minx, miny], [maxx, miny], [maxx, maxy], [minx, maxy], [minx, miny]];

        let bounding_box = Line::new(PlotPoints::from(path)).color(Color32::RED);

        plot_ui.line(bounding_box);
    }

    pub fn show_spatial_grid(&self, plot_ui: &mut PlotUi, simulation: &Simulation) {
        let grid = simulation.world.get_resource::<DenseGrid>().unwrap();

        let config = simulation.world.get_resource::<Config>().unwrap();
        let (minx, miny) = (config.bounds.0.x as f64, config.bounds.0.y as f64);
        let (maxx, maxy) = (config.bounds.1.x as f64, config.bounds.1.y as f64);

        // Draw vertical lines
        for i in 1..grid.ncells_side {
            let x = (i * grid.cell_size) as f64;
            let path = vec![[x, miny], [x, maxy]];
            let line = Line::new(PlotPoints::from(path)).color(Color32::LIGHT_BLUE);
            plot_ui.line(line);
        }

        // Draw horizontal lines
        for i in 1..grid.ncells_side {
            let y = (i * grid.cell_size) as f64;
            let path = vec![[minx, y], [maxx, y]];
            let line = Line::new(PlotPoints::from(path)).color(Color32::LIGHT_BLUE);
            plot_ui.line(line);
        }
    }

    fn build_base_plot(&self, display: &Display) -> Plot {
        let cam = &display.cam;

        let (minx, miny) = cam.screen2world(Vec2::zero()).into();
        let (maxx, maxy) = cam
            .screen2world(Vec2::new(display.cam.window_size.x, display.cam.window_size.y))
            .into();
        Plot::new("SimGrid")
            .show_background(false)
            .allow_zoom(true)
            .allow_drag(true)
            .data_aspect(1.0)
            .include_x(minx)
            .include_x(maxx)
            .include_y(miny)
            .include_y(maxy)
    }
}
