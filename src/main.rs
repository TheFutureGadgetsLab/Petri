#![feature(int_log)]
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

pub mod config;
pub mod rendering;
pub mod simulation;
pub mod timing;

use winit::{
    event::Event::*,
    event_loop::{ControlFlow, EventLoop},
};

use crate::{config::build_config, rendering::RenderDriver, simulation::Simulation};

fn main() {
    wgpu_subscriber::initialize_default_subscriber(None);

    let config = build_config();

    let mut simulation = Simulation::new(config);

    let event_loop = EventLoop::new();
    let mut renderer = RenderDriver::new(&mut simulation, &event_loop);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        // Forward event to renderers
        renderer.handle_event(&mut simulation, &event);

        match event {
            // Rendering
            RedrawRequested(..) => renderer.render(&simulation),
            // Updating simulation and queuing a redraw
            MainEventsCleared => {
                if !simulation.update() { // Exit if simulation should stop
                    *control_flow = ControlFlow::Exit;
                }
                renderer.request_render()
            }
            // Handle changes to wndow
            WindowEvent { event, .. } => renderer.handle_window_event(&event, control_flow),
            _ => {}
        }
    });
}
