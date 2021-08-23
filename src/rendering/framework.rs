// Heavily borrowed from Learn-WGPU
// https://github.com/sotrh/learn-wgpu/tree/master/code/showcase/framework

use wgpu::{SurfaceError, SurfaceFrame, TextureView};
use winit::event::Event::*;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window};
use fps_counter;

const INITIAL_WIDTH: u32 = 1920;
const INITIAL_HEIGHT: u32 = 1080;

use crate::simulation::{Config, Simulation};

pub struct Display {
    pub surface: wgpu::Surface,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub window: Window,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

impl Display {
    pub async fn new(window: Window) -> Display {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);
        let surface = unsafe { instance.create_surface(&window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::NON_FILL_POLYGON_MODE,
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Immediate,
        };

        Display {
            surface_config,
            surface,
            window,
            device,
            queue,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.surface_config.width  = width;
        self.surface_config.height = height;
        self.surface.configure(&self.device, &self.surface_config);
    }

    pub fn get_frame(&self) -> Result<(SurfaceFrame, TextureView), SurfaceError> {
        let output_frame = match self.surface.get_current_frame() {
            Ok(frame) => frame,
            Err(e) => {
                return Err(e);
            }
        };
        let output_view = output_frame
            .output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        Ok((output_frame, output_view))
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
    let window = winit::window::WindowBuilder::new()
        .with_decorations(true)
        .with_resizable(true)
        .with_transparent(false)
        .with_title("Petri")
        .with_inner_size(winit::dpi::PhysicalSize {
            width: INITIAL_WIDTH,
            height: INITIAL_HEIGHT,
        })
        .build(&event_loop)
        .unwrap();
    let mut display = Display::new(window).await;


    let mut app = Sim::init(&mut display);
    let mut gui = GUI::init(&mut display);

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
