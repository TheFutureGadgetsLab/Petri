use egui::{
    self,
    epaint::Color32,
    plot::{Line, Plot, Value, Values},
    Frame, InnerResponse, Ui,
};
use ultraviolet::Vec2;

use crate::{config::Config, rendering::Display, simulation::Simulation};

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
    pub fn update(&mut self, ctx: &egui::CtxRef, display: &Display, simulation: &Simulation) {
        egui::CentralPanel::default()
            .frame(
                Frame::dark_canvas(&ctx.style())
                    .fill(Color32::TRANSPARENT)
                    .margin([0.0, 0.0]),
            )
            .show(ctx, |ui| {
                self.show_sim_grid(ui, display, simulation);
            });
    }

    pub fn show_sim_grid(&self, ui: &mut Ui, display: &Display, simulation: &Simulation) -> InnerResponse<()> {
        let cam = &display.cam;

        let (minx, miny) = cam.screen2world(Vec2::zero()).into();
        let (maxx, maxy) = cam
            .screen2world(Vec2::new(
                display.surface_config.width as _,
                display.surface_config.height as _,
            ))
            .into();

        let config = simulation.resources.get::<Config>().unwrap();
        let path = [
            Value::new(config.bounds.0.x, config.bounds.0.y), // (xmin, ymin)
            Value::new(config.bounds.1.x, config.bounds.0.y), // (xmax, ymin)
            Value::new(config.bounds.1.x, config.bounds.1.y), // (xmax, ymax)
            Value::new(config.bounds.0.x, config.bounds.1.y), // (xmin, ymax)
            Value::new(config.bounds.0.x, config.bounds.0.y), // (xmin, ymin)
        ];

        let bounding_box = Line::new(Values::from_values(path.into())).color(Color32::RED);

        Plot::new("SimGrid")
            .show_background(false)
            .allow_zoom(true)
            .allow_drag(true)
            .data_aspect(1.0)
            .include_x(minx)
            .include_x(maxx)
            .include_y(miny)
            .include_y(maxy)
            .show(ui, |plot_ui| {
                plot_ui.line(bounding_box);
            })
    }
}
