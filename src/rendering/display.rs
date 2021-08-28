// Heavily borrowed from Learn-WGPU
// https://github.com/sotrh/learn-wgpu/tree/master/code/showcase/framework

use wgpu::{SurfaceError, SurfaceFrame, TextureView};
use winit::{window::Window, event_loop::EventLoop};
use futures::executor::block_on;

pub struct Display {
    pub surface: wgpu::Surface,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub window: Window,
}

const INITIAL_WIDTH: u32 = 1920;
const INITIAL_HEIGHT: u32 = 1080;

impl Display {
    pub fn new(event_loop: &EventLoop<()>) -> Display {
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

        
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);
        let surface = unsafe { instance.create_surface(&window) };
        let adapter = block_on(instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
            }))
            .unwrap();
        let (device, queue) = block_on(adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::NON_FILL_POLYGON_MODE,
                    limits: wgpu::Limits::default(),
                },
                None,
            ))
            .unwrap();

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Mailbox,
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