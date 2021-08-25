use wgpu::TextureView;
use winit::event::{Event, Event::*};
use winit::event_loop::{ControlFlow, EventLoop};
use fps_counter::{self, FPSCounter};

use crate::{rendering::{Display, GUIRenderer, SimRenderer}, simulation::{Config, Simulation}};

pub trait PetriEventLoop: 'static + Sized {
    fn init(display: &Display) -> Self;
    fn handle_event<T>(&mut self, display: &Display, event: &winit::event::Event<T>);
    fn update(&mut self, display: &Display);
    fn render(&mut self, display: &Display, simulation: &Simulation, view: &TextureView);
}

struct Package
{
    simulation:   Simulation,
    display:      Display,
    sim_renderer: SimRenderer,
    gui_renderer: GUIRenderer,

    tick: usize,
    fps_counter: FPSCounter,
}

impl Package {
    pub async fn new(config: Config, event_loop: &EventLoop<()>) -> Package {
        let simulation = Simulation::new(config);
        let display = Display::new(&event_loop).await;

        let sim_renderer = SimRenderer::init(&display);
        let gui_renderer = GUIRenderer::init(&display);

        let fps_counter = fps_counter::FPSCounter::default();
        let tick: usize = 0;

        Package {
            simulation,
            display,
            sim_renderer,
            gui_renderer,
            tick,
            fps_counter,
        }
    }

    pub fn handle_event(&mut self, event: &Event<()>) {
        self.sim_renderer.handle_event(&self.display, &event);
        self.gui_renderer.handle_event(&self.display, &event);
    }

    pub fn render(&mut self) {
        self.tick += 1;
        let fps = self.fps_counter.tick();
        if self.tick % 100 == 0 {
            println!("{}", fps);
        }
                
        self.sim_renderer.update(&self.display);
        self.gui_renderer.update(&self.display);

        let (_output_frame, output_view) = self.display.get_frame().unwrap();

        self.sim_renderer.render(&self.display, &self.simulation, &output_view);
        self.gui_renderer.render(&self.display, &self.simulation, &output_view);
    }

    pub fn request_render(&mut self) {
        self.simulation.update();
        self.display.window.request_redraw();
    }

    pub fn handle_window_event(&mut self, event: &winit::event::WindowEvent, control_flow: &mut ControlFlow) {
        match event {
            winit::event::WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            winit::event::WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                self.display.resize(new_inner_size.width, new_inner_size.height);
            }
            winit::event::WindowEvent::Resized(new_inner_size) => {
                self.display.resize(new_inner_size.width, new_inner_size.height);
            }
            _ => {}
        }
    }
}

pub async fn run(config: Config) {
    wgpu_subscriber::initialize_default_subscriber(None);

    let event_loop = EventLoop::new();
    let mut package = Package::new(config, &event_loop).await;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        // Forward event to renderers
        package.handle_event(&event);
        match event {
            // Rendering
            RedrawRequested(..) => package.render(),
            // Updating simulation and queuing a redraw
            MainEventsCleared => package.request_render(),
            // Handle changes to wndow
            WindowEvent { event, ..} => package.handle_window_event(&event, control_flow),
        _ => {}
        }
    });
}