// Heavily borrowed from Learn-WGPU
// https://github.com/sotrh/learn-wgpu/tree/master/code/showcase/framework

use std::time::{Duration, Instant};
use winit::event::*;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

pub struct Display {
    surface: wgpu::Surface,
    pub window: Window,
    pub sc_desc: wgpu::SwapChainDescriptor,
    pub swap_chain: wgpu::SwapChain,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

impl Display {
    pub async fn new(window: Window) -> Display {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
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
        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: adapter.get_swap_chain_preferred_format(&surface).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Immediate,
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        Display {
            surface,
            window,
            sc_desc,
            swap_chain,
            device,
            queue,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.sc_desc.width = width;
        self.sc_desc.height = height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }
}

pub trait PetriEventLoop: 'static + Sized {
    fn init(display: &Display) -> Self;
    fn process_mouse(&mut self, dx: f64, dy: f64);
    fn resize(&mut self, display: &Display);
    fn update(&mut self, display: &Display, dt: Duration);
    fn render(&mut self, display: &mut Display);
}

pub async fn run<D: PetriEventLoop>() {
    wgpu_subscriber::initialize_default_subscriber(None);

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(env!("CARGO_PKG_NAME"))
        .build(&event_loop).unwrap();
    let mut display = Display::new(window).await;
    let mut demo = D::init(&mut display);
    let mut last_update = Instant::now();
    let mut is_resumed = true;
    let mut is_focused = true;
    let mut is_redraw_requested = true;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = if is_resumed && is_focused {
            ControlFlow::Poll
        } else {
            ControlFlow::Wait
        };

        match event {
            Event::Resumed => is_resumed = true,
            Event::Suspended => is_resumed = false,
            Event::RedrawRequested(wid) => {
                if wid == display.window.id() {
                    let now = Instant::now();
                    let dt = now - last_update;
                    last_update = now;

                    demo.update(&mut display, dt);
                    demo.render(&mut display);
                    is_redraw_requested = false;
                }
            }
            Event::MainEventsCleared => {
                if is_focused && is_resumed && !is_redraw_requested {
                    display.window.request_redraw();
                    is_redraw_requested = true;
                } else {
                    // Freeze time while the demo is not in the foreground
                    last_update = Instant::now();
                }
            }
            Event::WindowEvent {
                event, window_id, ..
            } => {
                if window_id == display.window.id() {
                    match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::Focused(f) => is_focused = f,
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            display.resize(new_inner_size.width, new_inner_size.height);
                            demo.resize(&mut display);
                        }
                        WindowEvent::Resized(new_inner_size) => {
                            display.resize(new_inner_size.width, new_inner_size.height);
                            demo.resize(&mut display);
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    });
}
