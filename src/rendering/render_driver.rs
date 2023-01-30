use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{Event, MouseScrollDelta, WindowEvent::*},
    event_loop::{ControlFlow, EventLoop},
};

use crate::{
    config::Config,
    rendering::{Display, GUIRenderer, SimRenderer},
    simulation::{Simulation, Ticker},
};

pub struct RenderDriver {
    pub display: Display,
    pub sim_renderer: SimRenderer,
    pub gui_renderer: GUIRenderer,
    pub ticker: Ticker,
}

impl RenderDriver {
    pub fn new(simulation: &mut Simulation, event_loop: &EventLoop<()>) -> Self {
        let display = Display::new(event_loop);

        let sim_renderer = SimRenderer::new(&display, simulation);
        let gui_renderer = GUIRenderer::new(&display, simulation, event_loop);

        Self {
            display,
            sim_renderer,
            gui_renderer,
            ticker: Ticker::default(),
        }
    }

    pub fn handle_event(&mut self, simulation: &mut Simulation, event: &Event<()>) {
        self.sim_renderer.handle_event(&mut self.display, simulation, event);
        self.gui_renderer.handle_event(&mut self.display, simulation, event);
        self.display.handle_event(event);
    }

    pub fn render(&mut self, simulation: &mut Simulation) {
        self.ticker.tick();

        let (frame, view) = self.display.get_frame().unwrap();
        self.sim_renderer.render(&self.display, simulation, &view);
        self.gui_renderer.render(&self.display, simulation, &view);
        frame.present();
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

    pub fn should_redraw(&self, config: &Config) -> bool {
        let target_delta = 1.0 / (config.max_render_fps as f32);
        if self.ticker.delta_time().as_secs_f32() > target_delta {
            return true;
        }
        false
    }
}

pub trait PetriEventHandler {
    fn handle_event<T>(&mut self, display: &mut Display, simulation: &mut Simulation, event: &Event<T>) {
        self.forward_event(display, simulation, event);
        if let Event::WindowEvent { ref event, .. } = event {
            match event {
                Resized(size) => {
                    self.handle_resize(display, simulation, size);
                }
                MouseWheel {
                    delta: MouseScrollDelta::LineDelta(_, y),
                    ..
                } => {
                    self.handle_scroll(display, simulation, y);
                }
                CursorMoved { position, .. } => {
                    self.handle_mouse_move(display, simulation, position);
                }
                KeyboardInput { input, .. } => {
                    self.handle_keyboard_input(display, simulation, input);
                }
                _ => {}
            }
        }
    }

    fn forward_event<T>(&mut self, _display: &mut Display, _simulation: &mut Simulation, _event: &Event<T>) {}
    fn handle_resize(&mut self, _display: &mut Display, _simulation: &mut Simulation, _size: &PhysicalSize<u32>) {}
    fn handle_scroll(&mut self, _display: &mut Display, _simulation: &mut Simulation, _delta: &f32) {}
    fn handle_mouse_move(
        &mut self,
        _display: &mut Display,
        _simulation: &mut Simulation,
        _pos: &PhysicalPosition<f64>,
    ) {
    }
    fn handle_keyboard_input(
        &mut self,
        _display: &mut Display,
        _simulation: &mut Simulation,
        _input: &winit::event::KeyboardInput,
    ) {
    }
}
