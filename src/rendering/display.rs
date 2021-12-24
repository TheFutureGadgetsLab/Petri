// Heavily borrowed from Learn-WGPU
// https://github.com/sotrh/learn-wgpu/tree/master/code/showcase/framework

use futures::executor::block_on;
use ultraviolet::Vec2;
use wgpu::{SurfaceError, SurfaceTexture, TextureView};
use winit::{
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::EventLoop,
    window::Window,
};

use crate::rendering::sim_renderer::camera::Camera;

#[derive(PartialEq, Eq, Clone, Copy, Default)]
pub struct InputState {
    pub pressed: bool,
    pub released: bool,
    pub held: bool,
}

pub struct Mouse {
    pub pos: Vec2,
    pub delta: Vec2,
    /// Left, middle, right
    pub buttons: [InputState; 3],
}

pub struct Display {
    pub surface: wgpu::Surface,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub window: Window,

    pub cam: Camera,

    pub mouse: Mouse,
}

const INITIAL_SIZE: Vec2 = Vec2 { x: 1920.0, y: 1080.0 };

impl Display {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        let window = winit::window::WindowBuilder::new()
            .with_decorations(true)
            .with_resizable(true)
            .with_transparent(false)
            .with_title("Petri")
            .with_inner_size(winit::dpi::PhysicalSize::new(INITIAL_SIZE.x, INITIAL_SIZE.y))
            .build(event_loop)
            .unwrap();

        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::Backends::VULKAN);
        let surface = unsafe { instance.create_surface(&window) };
        let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .unwrap();
        let (device, queue) = block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::POLYGON_MODE_POINT,
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
            present_mode: wgpu::PresentMode::Immediate,
        };

        Self {
            surface_config,
            surface,
            window,
            device,
            queue,
            cam: Camera::new(INITIAL_SIZE),
            mouse: Mouse {
                pos: Vec2::zero(),
                delta: Vec2::zero(),
                buttons: [InputState::default(); 3],
            },
        }
    }

    pub fn handle_event(&mut self, event: &Event<()>) {
        if let Event::WindowEvent { ref event, .. } = event {
            match event {
                WindowEvent::MouseInput { button, state, .. } => {
                    let mut butt_idx = 0usize;
                    match button {
                        MouseButton::Left => butt_idx = 0,
                        MouseButton::Right => butt_idx = 1,
                        MouseButton::Middle => butt_idx = 2,
                        _ => {}
                    }
                    let mut bstate = self.mouse.buttons[butt_idx];
                    match state {
                        ElementState::Pressed => {
                            if bstate.held {
                                bstate.pressed = false;
                            } else {
                                bstate.pressed = true;
                                bstate.held = true;
                            }
                        }
                        ElementState::Released => {
                            bstate.pressed = false;
                            bstate.held = false;
                            if bstate.released {
                                bstate.released = false;
                            }
                        }
                    }
                    self.mouse.buttons[butt_idx] = bstate;
                }
                WindowEvent::CursorMoved { position, .. } => {
                    let pos = Vec2::new(position.x as f32, position.y as f32);
                    self.mouse.delta = pos - self.mouse.pos;
                    self.mouse.pos = pos;
                }
                _ => {}
            }
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.surface_config.width = width;
        self.surface_config.height = height;
        self.surface.configure(&self.device, &self.surface_config);
    }

    pub fn get_frame(&self) -> Result<(SurfaceTexture, TextureView), SurfaceError> {
        let frame = match self.surface.get_current_texture() {
            Ok(frame) => frame,
            Err(_) => {
                self.surface.configure(&self.device, &self.surface_config);
                self.surface
                    .get_current_texture()
                    .expect("Failed to acquire next surface texture!")
            }
        };
        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
        Ok((frame, view))
    }
}
