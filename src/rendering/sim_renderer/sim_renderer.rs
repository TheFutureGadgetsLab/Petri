use std::time::Instant;

use bytemuck;
use glam::vec2;
use shaderc::CompileOptions;
use wgpu::{ShaderModuleDescriptor, TextureView};
use winit::event::VirtualKeyCode;

use super::{camera::Camera, Vertex, VertexBuffer};
use crate::{
    rendering::{Display, PetriEventHandler},
    simulation::Simulation,
    timing::TIMING_DATABASE,
};

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform {
    u_translation: [f32; 2],
    u_window_size: [f32; 2],
    u_zoom: [f32; 2],
}

impl From<&Camera> for CameraUniform {
    fn from(cam: &Camera) -> Self {
        CameraUniform {
            u_translation: cam.translation.into(),
            u_window_size: cam.window_size.into(),
            u_zoom: [cam.zoom; 2],
        }
    }
}

pub struct SimRenderer {
    uniforms_ubo: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: VertexBuffer,
}

impl SimRenderer {
    pub fn new(display: &Display, _simulation: &mut Simulation) -> Self {
        let uniforms_buffer_byte_size = std::mem::size_of::<CameraUniform>() as wgpu::BufferAddress;
        let uniforms_ubo = display.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Uniforms ubo"),
            size: uniforms_buffer_byte_size,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group_layout = display
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Bind group layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(uniforms_buffer_byte_size),
                    },
                    count: None,
                }],
            });
        let bind_group = display.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Bind group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(uniforms_ubo.as_entire_buffer_binding()),
            }],
        });
        let mut options = CompileOptions::new().unwrap();
        options.set_optimization_level(shaderc::OptimizationLevel::Performance);

        let mut compiler = shaderc::Compiler::new().unwrap();
        let vert_comp = compiler
            .compile_into_spirv(
                include_str!("shaders/particles.vert"),
                shaderc::ShaderKind::Vertex,
                "shaders/particles.vert",
                "main",
                Some(&options),
            )
            .unwrap();
        let frag_comp = compiler
            .compile_into_spirv(
                include_str!("shaders/particles.frag"),
                shaderc::ShaderKind::Fragment,
                "shaders/particles.frag",
                "main",
                Some(&options),
            )
            .unwrap();

        let shader_frag = &display.device.create_shader_module(&ShaderModuleDescriptor {
            label: Some("Fragment shader"),
            source: wgpu::util::make_spirv(frag_comp.as_binary_u8()),
        });
        let shader_vert = &display.device.create_shader_module(&ShaderModuleDescriptor {
            label: Some("Vertex shader"),
            source: wgpu::util::make_spirv(vert_comp.as_binary_u8()),
        });

        // Create render pipeline
        let render_pipeline_layout = display.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });
        let render_pipeline = display.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: shader_vert,
                entry_point: "main",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x4, 2 => Float32],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: shader_frag,
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
            multisample: wgpu::MultisampleState::default(),
        });

        Self {
            uniforms_ubo,
            bind_group,
            render_pipeline,
            vertex_buffer: VertexBuffer::default(display),
        }
    }

    pub fn render(&mut self, display: &Display, simulation: &Simulation, view: &TextureView) {
        let start = Instant::now();
        let cam_uniform = CameraUniform::from(&display.cam);
        display
            .queue
            .write_buffer(&self.uniforms_ubo, 0, bytemuck::cast_slice(&[cam_uniform]));

        let mut encoder = display.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        let n_vertices = self.vertex_buffer.update(display, simulation);

        {
            // Set up render pass and associate the render pipeline we made
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
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

        TIMING_DATABASE.write().sim_render.render.update(Instant::now() - start)
    }
}

impl PetriEventHandler for SimRenderer {
    fn handle_resize(
        &mut self,
        display: &mut Display,
        _simulation: &mut Simulation,
        size: &winit::dpi::PhysicalSize<u32>,
    ) {
        display.cam.resize(size.width as _, size.height as _);
    }

    fn handle_scroll(&mut self, display: &mut Display, _simulation: &mut Simulation, delta: &f32) {
        display.cam.zoom *= 1.0 + delta.signum() * 0.1;
    }

    fn handle_mouse_move(
        &mut self,
        display: &mut Display,
        _simulation: &mut Simulation,
        _pos: &winit::dpi::PhysicalPosition<f64>,
    ) {
        if display.mouse.buttons[0].held {
            display.cam.translate_by(display.mouse.delta * vec2(1.0, -1.0));
        }
    }

    fn handle_keyboard_input(
        &mut self,
        display: &mut Display,
        _simulation: &mut Simulation,
        input: &winit::event::KeyboardInput,
    ) {
        if input.virtual_keycode.is_some() {
            match input.virtual_keycode.unwrap() {
                VirtualKeyCode::Left => display.cam.translate_by([1.0, 0.0].into()),
                VirtualKeyCode::Right => display.cam.translate_by([-1.0, 0.0].into()),
                VirtualKeyCode::Up => display.cam.translate_by([0.0, -1.0].into()),
                VirtualKeyCode::Down => display.cam.translate_by([0.0, 1.0].into()),
                _ => {}
            }
        }
    }
}
