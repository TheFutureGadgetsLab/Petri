use crate::{driver::{PetriEventLoop, Stats}, rendering::Display, simulation::Simulation};

use super::{VertexBuffer, Vertex, camera::Camera};
use glam::Vec2;
use winit::{event::{VirtualKeyCode, ElementState, Event, MouseButton, WindowEvent}};

use wgpu::{ShaderModuleDescriptor, TextureView};
use bytemuck;
use shaderc::CompileOptions;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Globals {
    res: [f32; 2],
}

pub struct SimRenderer {
    globals_ubo: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: VertexBuffer,
    cam: Camera,
    prev_mouse_pos: Vec2,
    mouse_pos: Vec2,
    mouse_drag_start: Vec2,
    mouse_click: bool
}

impl PetriEventLoop for SimRenderer {
    fn init(display: &Display) -> SimRenderer {
        let globals_buffer_byte_size = std::mem::size_of::<Globals>() as wgpu::BufferAddress;
        let globals_ubo = display.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Globals ubo"),
            size: globals_buffer_byte_size,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group_layout = display.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Bind group layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(globals_buffer_byte_size),
                    },
                    count: None,
                },
            ],
        });
        let bind_group = display.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Bind group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(globals_ubo.as_entire_buffer_binding()),
                },
            ],
        });
        let mut options = CompileOptions::new().unwrap();
        options.set_optimization_level(shaderc::OptimizationLevel::Performance);

        let mut compiler = shaderc::Compiler::new().unwrap();
        let vert_comp = compiler.compile_into_spirv(
            &include_str!("shaders/particles.vert"),
            shaderc::ShaderKind::Vertex,
            &"shaders/particles.vert",
            "main",
            Some(&options),
        ).unwrap();
        let frag_comp = compiler.compile_into_spirv(
            &include_str!("shaders/particles.frag"),
            shaderc::ShaderKind::Fragment,
            &"shaders/particles.frag",
            "main",
            Some(&options),
        ).unwrap();

        let shader_frag = &display.device.create_shader_module(
            &ShaderModuleDescriptor {
                    label: Some("Fragment shader"),
                    source: wgpu::util::make_spirv(frag_comp.as_binary_u8()),
            }
        );
        let shader_vert = &display.device.create_shader_module(
            &ShaderModuleDescriptor {
                    label: Some("Vertex shader"),
                    source: wgpu::util::make_spirv(vert_comp.as_binary_u8()),
            }
        );

        // Create render pipeline
        let render_pipeline_layout =
            display.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            });
        let render_pipeline = display.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader_vert,
                entry_point: "main",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x4, 2 => Float32]
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader_frag,
                entry_point: "main",
                targets: &[wgpu::ColorTargetState {
                    format: display.surface_config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::PointList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Point,
                // Requires Features::DEPTH_CLAMPING
                clamp_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default()
        });

        SimRenderer {
            globals_ubo,
            bind_group,
            render_pipeline,
            vertex_buffer: VertexBuffer::default(display),
            cam: Camera::new(display),
            prev_mouse_pos: Vec2::ZERO,
            mouse_pos: Vec2::ZERO,
            mouse_drag_start: Vec2::ZERO,
            mouse_click: false,
        }
    }

    fn handle_event<T>(&mut self, display: &Display, event: &Event<T>) {
        // Need to handle scale factor change
        match event {
            Event::WindowEvent { ref event, ..}  => {
                match event {
                    WindowEvent::Resized(_) => {
                        let size = display.window.inner_size();
                        self.cam.size = [size.width as f32, size.height as f32].into();
                        display.queue.write_buffer(&self.globals_ubo, 0, bytemuck::cast_slice(&[Globals {
                            res: self.cam.size.into()
                        }]));
                    }
                    WindowEvent::MouseInput {button, state, ..} => {
                        match button {
                            MouseButton::Left => {
                                match state {
                                    ElementState::Pressed => {
                                        self.mouse_click = true;
                                        self.mouse_drag_start = self.cam.screen2world(self.mouse_pos) + self.cam.pos;
                                    }
                                    ElementState::Released => { self.mouse_click = false; }
                                }
                            }
                            _ => {}
                        }
                    }
                    WindowEvent::CursorMoved {position, ..} => {
                        self.prev_mouse_pos = self.mouse_pos;
                        self.mouse_pos.x = position.x as f32;
                        self.mouse_pos.y = position.y as f32;
                        if self.mouse_click {
                            self.cam.pos = self.mouse_drag_start - self.cam.screen2world(self.mouse_pos);
                        }
                    }
                    WindowEvent::KeyboardInput { input , ..} => {
                        if input.virtual_keycode.is_some() {
                            match input.virtual_keycode.unwrap() {
                                VirtualKeyCode::Left =>     { self.cam.pos.x -= 20.0; }
                                VirtualKeyCode::Right =>    { self.cam.pos.x += 20.0; }
                                VirtualKeyCode::Up =>       { self.cam.pos.y -= 20.0; }
                                VirtualKeyCode::Down =>     { self.cam.pos.y += 20.0; }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    fn update(&mut self, _display: &Display) {
    }

    fn render(&mut self, display: &Display, simulation: &Simulation, view: &TextureView, _stats: &Stats) {
        let mut encoder = display.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        let n_vertices = self.vertex_buffer.update(&display, &simulation, &self.cam);

        { // Set up render pass and associate the render pipeline we made
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[
                    wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.0,
                                g: 0.0,
                                b: 0.0,
                                a: 1.0,
                            }),
                            store: true,
                        }
                    }
                ],
                depth_stencil_attachment: None,
            });
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.buf.slice(..));
            render_pass.set_bind_group(0, &self.bind_group, &[]);
            render_pass.draw(0..n_vertices, 0..1);
        }
    
        // Submit will accept anything that implements IntoIter
        // Submits the command buffer
        display.queue.submit(std::iter::once(encoder.finish()));
    }
}