use egui::{epaint::Color32, plot::Line, Frame};
use glam::{vec2, Vec2};

use crate::{rendering::Display, simulation::Simulation};

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
    pub fn update(&mut self, ctx: &egui::CtxRef, display: &Display, _simulation: &Simulation) {
        let cam = &display.cam;

        let (minx, miny) = cam.screen2world(Vec2::ZERO).into();
        let (maxx, maxy) = cam
            .screen2world(vec2(
                display.surface_config.width as _,
                display.surface_config.height as _,
            ))
            .into();

        egui::CentralPanel::default()
            .frame(
                Frame::dark_canvas(&ctx.style())
                    .fill(Color32::TRANSPARENT)
                    .margin([0.0, 0.0]),
            )
            .show(ctx, |ui| {
                ui.add(
                    Plot::new("test")
                        .show_background(false)
                        .allow_zoom(false)
                        .allow_drag(false)
                        .data_aspect(1.0)
                        .include_x(minx)
                        .include_x(maxx)
                        .include_y(miny)
                        .include_y(maxy),
                );
            });
    }
}
