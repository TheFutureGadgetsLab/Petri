mod rendering;
mod simulation;

use winit::event::Event::*;
use winit::event_loop::{ControlFlow, EventLoop};
use crate::rendering::RenderDriver;
use crate::simulation::{Config, Simulation};

fn main() {
    wgpu_subscriber::initialize_default_subscriber(None);

    let config = Config::default();
    let mut simulation = Simulation::new(config);

    let event_loop = EventLoop::new();
    let mut renderer = RenderDriver::new(&simulation, &event_loop);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        // Forward event to renderers
        renderer.handle_event(&event);

        match event {
            // Rendering
            RedrawRequested(..) => renderer.render(&simulation),
            // Updating simulation and queuing a redraw
            MainEventsCleared => {
                simulation.update();
                renderer.request_render()
            },
            // Handle changes to wndow
            WindowEvent { event, ..} => renderer.handle_window_event(&event, control_flow),
        _ => {}
        }
    });
}