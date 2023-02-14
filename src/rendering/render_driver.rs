use winit::{
    event::WindowEvent::*,
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

        let mut gui_renderer = GUIRenderer::new(&display, simulation, event_loop);
        let sim_renderer = SimRenderer::new(&display, simulation, &mut gui_renderer.rpass);

        Self {
            display,
            sim_renderer,
            gui_renderer,
            ticker: Ticker::default(),
        }
    }

    pub fn render(&mut self, simulation: &mut Simulation) {
        self.ticker.tick();

        let (frame, view) = self.display.get_frame().unwrap();

        self.gui_renderer.pre_render(&self.display);
        self.gui_renderer
            .render(&mut self.display, simulation, &mut self.sim_renderer);
        self.gui_renderer.post_render(&self.display, &view);

        frame.present();
    }

    pub fn request_render(&mut self) {
        self.display.window.request_redraw();
    }

    pub fn should_redraw(&self, config: &Config) -> bool {
        let target_delta = 1.0 / (config.max_render_fps as f32);
        if self.ticker.delta_time().as_secs_f32() > target_delta {
            return true;
        }
        false
    }
}

// Event handling
impl RenderDriver {
    pub fn handle_window_event(&mut self, event: &winit::event::WindowEvent, control_flow: &mut ControlFlow) {
        let response = self.gui_renderer.state.on_event(&self.gui_renderer.context, event);
        if response.consumed {
            return;
        }

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
