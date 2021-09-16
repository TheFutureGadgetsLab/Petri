use crate::{
    rendering::Display,
    simulation::{Simulation, Time},
    timing::{timer::Timer, TIMING_DATABASE},
};

pub struct PerfApp;

impl PerfApp {
    pub fn update(&mut self, ctx: &egui::CtxRef, _display: &Display, simulation: &Simulation) {
        // Reset timer at 10th tick to ignore startup lag
        if simulation.resources.get::<Time>().unwrap().tick == 10 {
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
        let mut kv: Vec<(&str, &str, &Timer)> = database
            .iter()
            .map(|(name, timer)| {
                let splits: Vec<&str> = name.split(".").collect();
                (*splits.get(0).unwrap(), *splits.get(1).unwrap(), timer)
            })
            .collect();

        // sort by system, then duration (longest first)
        kv.sort_by(|(an, _, at), (bn, _, bt)| {
            if an == bn {
                return bt.timer.mean().partial_cmp(&at.timer.mean()).unwrap();
            }
            an.cmp(bn)
        });

        let mut prev_heading = "";
        for (name, stage, timer) in kv.iter() {
            if name != &prev_heading {
                ui.heading(name.to_string());
                ui.separator();
                prev_heading = name;
            }

            ui.label(format!("{} {}", stage, timer.res_str));
            ui.label(format!("\t{}", timer));
        }
    }
}
