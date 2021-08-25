use wgpu::TextureView;
use winit::event::Event::*;
use winit::event_loop::{ControlFlow, EventLoop};
use fps_counter;

use crate::{rendering::{Display, GUIRenderer, SimRenderer}, simulation::{Config, Simulation}};

pub struct Driver
{
    pub display: Display,
    
    pub sim_renderer: SimRenderer,
    pub gui_renderer: GUIRenderer,
    
    pub config: Config,
    pub simulation: Simulation,

    pub event_loop: EventLoop<()>
}

impl Driver {
    pub fn new() {

    }
}



pub trait PetriEventLoop: 'static + Sized {
    fn init(display: &Display) -> Self;
    fn handle_event<T>(&mut self, display: &Display, event: &winit::event::Event<T>);
    fn update(&mut self, display: &Display);
    fn render(&mut self, display: &Display, simulation: &Simulation, view: &TextureView);
}

pub async fn run<Sim: PetriEventLoop, GUI: PetriEventLoop>(config: Config) {
    wgpu_subscriber::initialize_default_subscriber(None);

    let mut simulation = Simulation::new(config);

    let mut fps_counter = fps_counter::FPSCounter::default();
    let mut tick: usize = 0;

    let event_loop = EventLoop::new();
    let mut display = Display::new(&event_loop).await;


    let mut app = Sim::init(&display);
    let mut gui = GUI::init(&display);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        app.handle_event(&display, &event);
        gui.handle_event(&display, &event);
        match event {
            // Rendering
            RedrawRequested(..) => {
                tick += 1;
                let fps = fps_counter.tick();
                if tick % 100 == 0 {
                    println!("{}", fps);
                }

                app.update(&display);
                gui.update(&display);

                let (_output_frame, output_view) = display.get_frame().unwrap();

                app.render(&display, &simulation, &output_view);
                gui.render(&display, &simulation, &output_view);
            }
            // Updating simulation and queuing a redraw
            MainEventsCleared => {
                simulation.update();
                display.window.request_redraw();
            }
            WindowEvent {
                event, ..
            } => {
                match event {
                    winit::event::WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    winit::event::WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        display.resize(new_inner_size.width, new_inner_size.height);
                    }
                    winit::event::WindowEvent::Resized(new_inner_size) => {
                        display.resize(new_inner_size.width, new_inner_size.height);
                    }
                    _ => {}
                }
            }
        _ => {}
        }
    });
}