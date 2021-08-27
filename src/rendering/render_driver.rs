use crate::{
    rendering::{
        Display,
        GUIRenderer,
        SimRenderer
    }, 
    simulation::{
        Simulation
    }
};

use winit::event::WindowEvent::*;
use winit::event::Event;
use winit::event_loop::{ControlFlow, EventLoop};
use wgpu::TextureView;

pub trait PetriEventLoop: 'static + Sized {
    fn init(display: &Display, simulation: &Simulation) -> Self;
    fn handle_event<T>(&mut self, display: &Display, event: &winit::event::Event<T>);
    fn update(&mut self, display: &Display);
    fn render(&mut self, display: &Display, simulation: &Simulation, view: &TextureView);
}

pub struct RenderDriver
{
    pub display:      Display,
    pub sim_renderer: SimRenderer,
    pub gui_renderer: GUIRenderer,
}

impl RenderDriver {
    pub fn new(simulation: &Simulation, event_loop: &EventLoop<()>) -> Self {
        let display = Display::new(&event_loop);

        let sim_renderer = SimRenderer::init(&display, &simulation);
        let gui_renderer = GUIRenderer::init(&display, &simulation);

        Self {
            display,
            sim_renderer,
            gui_renderer,
        }
    }

    pub fn handle_event(&mut self, event: &Event<()>) {
        self.sim_renderer.handle_event(&self.display, &event);
        self.gui_renderer.handle_event(&self.display, &event);
    }

    pub fn render(&mut self, simulation: &Simulation) {
        self.sim_renderer.update(&self.display);
        self.gui_renderer.update(&self.display);

        let (_output_frame, output_view) = self.display.get_frame().unwrap();

        self.sim_renderer.render(&self.display, simulation, &output_view);
        self.gui_renderer.render(&self.display, simulation, &output_view);
    }

    pub fn request_render(&mut self) {
        self.display.window.request_redraw();
    }

    pub fn handle_window_event(&mut self, event: &winit::event::WindowEvent, control_flow: &mut ControlFlow) {
        match event {
            CloseRequested => *control_flow = ControlFlow::Exit,
            ScaleFactorChanged { new_inner_size, .. } => {
                self.display.resize(new_inner_size.width, new_inner_size.height);
            }
            Resized(new_inner_size) => {
                self.display.resize(new_inner_size.width, new_inner_size.height);
            }
            _ => {}
        }
    }
}
