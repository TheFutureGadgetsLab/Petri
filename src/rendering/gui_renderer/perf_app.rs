use crate::{rendering::Display, simulation::Simulation, timing::TIMING_DATABASE};

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct PerfApp;

impl PerfApp {
    pub fn update(&mut self, ctx: &egui::CtxRef, _display: &Display, _simulation: &Simulation) {
        egui::SidePanel::right("Performance Info").show(ctx, |ui| {
            ui.style_mut().wrap = Some(false);
            ui.heading("Performance");
            self.physics(ui);
            self.sim_render(ui);
        });
    }

    fn physics(&self, ui: &mut egui::Ui) {
        let database = TIMING_DATABASE.read();

        ui.separator();
        ui.heading("Physics");

        ui.label(format!("Full Step {}", database.physics.step.res_str));
        ui.label(format!("\t{}", database.physics.step));

        ui.label(format!("Pos Update {}", database.physics.pos_update.res_str));
        ui.label(format!("\t{}", database.physics.pos_update));

        ui.label(format!("Col Detection {}", database.physics.col_detect.res_str));
        ui.label(format!("\t{}", database.physics.col_detect));

        ui.label(format!("Col Resolve {}", database.physics.col_resolve.res_str));
        ui.label(format!("\t{}", database.physics.col_resolve));
    }

    fn sim_render(&self, ui: &mut egui::Ui) {
        let database = TIMING_DATABASE.read();

        ui.separator();
        ui.heading("Sim Render");

        ui.label(format!(
            "Vertex Update {}",
            database.sim_render.vertex_buffer_update.res_str
        ));
        ui.label(format!("\t{}", database.sim_render.vertex_buffer_update));

        ui.label(format!("Render {}", database.sim_render.render.res_str));
        ui.label(format!("\t{}", database.sim_render.render));
    }
}
