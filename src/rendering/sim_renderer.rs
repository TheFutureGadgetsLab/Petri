use crate::rendering::framework::{
    PetriEventLoop, Display
};

use std::time::Duration;
use bytemuck;
use fps_counter;
use wgpu::util::DeviceExt;
use rayon::prelude::*;


#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Globals {
    res: [f32; 2],
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 4],
    size: f32
}

pub struct SimRenderer {
    globals_ubo: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    vertices: Vec<Vertex>,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    time: f32,
    i: i32,
    fps_counter: fps_counter::FPSCounter,
}

const N_PARTICLES: usize = 3_000_000;

impl PetriEventLoop for SimRenderer {
    fn init(display: &Display) -> SimRenderer {
        let mut vertices = vec![Vertex {position: [0.0; 2], color: [0.0; 4], size: 1.0}; N_PARTICLES];
        for (i, v) in vertices.iter_mut().enumerate() {
            let i = i as f32;
            v.position = [
                (i * 0.171982347).cos(),
                (0.612834028 + i * 0.131234892).sin()];
            v.color = [(i * 0.1416) % 1.0, (i * 0.336) % 1.0, (i * 0.0714) % 1.0, 1.0];
            v.size = 3.0 + 8.0 * (i.cos() * 0.5 + 0.5);
        }

        let vertex_buffer = display.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsage::VERTEX | wgpu::BufferUsage::COPY_DST
        });

        let globals_buffer_byte_size = std::mem::size_of::<Globals>() as wgpu::BufferAddress;
        let globals_ubo = display.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Globals ubo"),
            size: globals_buffer_byte_size,
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group_layout = display.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Bind group layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::FRAGMENT,
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

        let shader_vert = &display.device.create_shader_module(&wgpu::include_spirv!("../shaders/particles.vert.spv"));
        let shader_frag = &display.device.create_shader_module(&wgpu::include_spirv!("../shaders/particles.frag.spv"));

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
                    step_mode: wgpu::InputStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x4, 2 => Float32]
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader_frag,
                entry_point: "main",
                targets: &[wgpu::ColorTargetState {
                    format: display.sc_desc.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrite::ALL,
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
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
        });

        SimRenderer {
            globals_ubo: globals_ubo,
            bind_group: bind_group,
            vertices: vertices,
            render_pipeline: render_pipeline,
            vertex_buffer: vertex_buffer,
            time: 0.,
            i: 0,
            fps_counter: fps_counter::FPSCounter::default(),
        }
    }

    fn process_mouse(&mut self, _dx: f64, _dy: f64) {
    }

    fn resize(&mut self, display: &Display) {
        let size = display.window.inner_size();
        display.queue.write_buffer(&self.globals_ubo, 0, bytemuck::cast_slice(&[Globals {
            res: [size.width as f32, size.height as f32]
        }]));
    }

    fn update(&mut self, display: &Display, _dt: Duration) {
        self.time += 0.0005;
        let t = self.time;
        self.vertices.par_chunks_mut(4096).for_each(|vs| {
            for (i, v) in vs.iter_mut().enumerate() {
                let i = i as f32;
                v.position = [
                    (t * v.color[0] * (v.color[2] - 0.5) * 1.13 + i * 0.171982347).cos(),
                    (t * v.color[1] * (v.color[3] - 0.5) * 2.31 + 0.612834028 + i * 0.131234892).sin()];
            }
        });
        display.queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&self.vertices));
    }

    fn render(&mut self, display: &mut Display) {
        self.i += 1;
        let fps = self.fps_counter.tick();
        if self.i % 100 == 0 {
            println!("{}", fps);
        }
        let frame = display
            .swap_chain
            .get_current_frame().unwrap()
            .output;
        let mut encoder = display.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });


        { // Set up render pass and associate the render pipeline we made
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[
                    wgpu::RenderPassColorAttachment {
                        view: &frame.view,
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
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_bind_group(0, &self.bind_group, &[]);
            render_pass.draw(0..(N_PARTICLES as u32), 0..1);
        }
    
        // Submit will accept anything that implements IntoIter
        // Submits the command buffer
        display.queue.submit(std::iter::once(encoder.finish()));
    }
}