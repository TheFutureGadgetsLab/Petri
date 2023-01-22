use std::collections::HashMap;

use crate::{
    rendering::Display,
    simulation::{Simulation, Time},
    timing::{timer::Timer, TIMING_DATABASE},
};

pub struct PerfApp;

impl PerfApp {
    pub fn update(&mut self, ctx: &egui::Context, _display: &Display, simulation: &Simulation) {
        // Reset timer at 10th tick to ignore startup lag
        if simulation.resources.get::<Time>().unwrap().tick == 100 {
            for v in TIMING_DATABASE.write().values_mut() {
                v.reset();
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
            stages.sort_by_key(|(_, timer)| timer.raw_imean());
            stages.reverse();
            egui::Grid::new(format!("{sys:} Grid"))
                .striped(true)
                .num_columns(4)
                .show(ui, |ui| {
                    ui.heading(*sys);
                    ui.heading("Mean");
                    ui.heading("Min");
                    ui.heading("Max");
                    ui.end_row();
                    for (stage, timer) in stages.iter() {
                        draw_system(ui, stage, timer);
                        ui.end_row();
                    }
                });

            if i != vec.len() - 1 {
                ui.separator();
            }
        }
    }
}

// Function to draw the performance info for one system
fn draw_system(ui: &mut egui::Ui, stage: &str, timer: &Timer) {
    let (mean, mean_str) = timer.mean();
    let (min, min_str) = timer.min();
    let (max, max_str) = timer.max();
    ui.label(stage);
    ui.label(format!("{mean} ({mean_str})"));
    ui.label(format!("{min} ({min_str})"));
    ui.label(format!("{max} ({max_str})"));
}
