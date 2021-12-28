use std::collections::HashMap;

use crate::{
    rendering::Display,
    simulation::{Simulation, Time},
    timing::TIMING_DATABASE,
};

pub struct PerfApp;

impl PerfApp {
    pub fn update(&mut self, ctx: &egui::CtxRef, _display: &Display, simulation: &Simulation) {
        // Reset timer at 10th tick to ignore startup lag
        if simulation.resources.get::<Time>().unwrap().tick == 100 {
            for v in TIMING_DATABASE.write().values_mut() {
                v.timer.reset();
            }
        }

        egui::SidePanel::right("Performance Info").show(ctx, |ui| {
            ui.style_mut().wrap = Some(false);
            self.draw(ui);
        });
    }

    fn draw(&self, ui: &mut egui::Ui) {
        let database = TIMING_DATABASE.read();
        // Map to (system, stage, timer)

        let mut map = HashMap::new();
        for (string, timers) in database.iter() {
            let splits: Vec<&str> = string.split('.').collect();
            let system = splits[0];
            let stage = splits[1];

            map.entry(system).or_insert_with(Vec::default).push((stage, timers));
        }

        // collect into vector of [(system, (stage, timer)), ..]
        let mut vec = Vec::new();
        for (system, stages) in map.iter() {
            vec.push((*system, stages.clone()));
        }

        // sort by system
        vec.sort_by(|a, b| a.0.cmp(b.0));

        for (i, (sys, stages)) in vec.clone().iter_mut().enumerate() {
            // sort stages by timer mean
            stages.sort_by(|a, b| b.1.mean().partial_cmp(&a.1.mean()).unwrap());
            egui::Grid::new(format!("{:} Grid", sys))
                .striped(true)
                .num_columns(4)
                .show(ui, |ui| {
                    ui.heading(sys);
                    ui.heading("Mean");
                    ui.heading("Min");
                    ui.heading("Max");
                    ui.end_row();
                    for (stage, timer) in stages.iter() {
                        ui.label(stage);
                        ui.label(format!("{:.2}", timer.timer.mean()));
                        ui.label(format!("{:.2}", timer.timer.min()));
                        ui.label(format!("{:.2}", timer.timer.max()));
                        ui.end_row();
                    }
                });

            if i != vec.len() - 1 {
                ui.separator();
            }
        }
    }
}
