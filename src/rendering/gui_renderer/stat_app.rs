use egui::Frame;
use egui::epaint::Color32;
use glam::{vec2, Vec2};

use crate::{rendering::Display, simulation::{Config, Simulation, Time}};

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct StatApp;

impl StatApp {
    pub fn update(&mut self, ctx: &egui::CtxRef, display: &Display, simulation: &Simulation) {
        let time = simulation.resources.get::<Time>().unwrap();
        let config = simulation.resources.get::<Config>().unwrap();
        let cam = &display.cam;

        let (minx, miny) = cam.screen2world(Vec2::ZERO).into();
        let (maxx, maxy) = cam.screen2world(vec2(display.surface_config.width as _, display.surface_config.height as _)).into();

        egui::CentralPanel::default()
            .frame(Frame::dark_canvas(&ctx.style()).fill(Color32::TRANSPARENT).margin([0.0, 0.0]))
            .show(ctx, |ui| {
                ui.add(
                 egui::plot::Plot::new("test")
                    .show_background(false)
                    .allow_zoom(false)
                    .allow_drag(false)
                    .data_aspect(1.0)
                    .include_x(minx)
                    .include_x(maxx)
                    .include_y(miny)
                    .include_y(maxy)
                );
            });


        egui::SidePanel::left("Debug Info").show(ctx, |ui| {
            ui.style_mut().wrap = Some(false);
            ui.heading("Stats");
            ui.label(format!("Time: {:.2}", time.time_since_start().as_secs_f32()));
            ui.label(format!("Ticks / s: {:}", time.tick_rate));
            ui.label(format!("Entities: {}", simulation.world.len()));

            ui.separator();

            ui.heading("Camera");
            ui.label(format!(
                "Position: ({:.2}, {:.2})",
                cam.pos().x, cam.pos().y
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
