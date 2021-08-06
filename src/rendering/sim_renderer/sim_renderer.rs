use crate::{
    rendering::{
        framework::{
            PetriEventLoop, Display
        }, 
        sim_renderer::{VertexBuffer, Vertex},
    },
    simulation::Simulation
};

use wgpu::{ShaderModuleDescriptor};
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
}

impl PetriEventLoop for SimRenderer {
    fn init(display: &Display) -> SimRenderer {
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
                    flags: wgpu::ShaderFlags::VALIDATION,
            }
        );
        let shader_vert = &display.device.create_shader_module(
            &ShaderModuleDescriptor {
                    label: Some("Vertex shader"),
                    source: wgpu::util::make_spirv(vert_comp.as_binary_u8()),
                    flags: wgpu::ShaderFlags::VALIDATION,
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
            render_pipeline: render_pipeline,
            vertex_buffer: VertexBuffer::default(display),
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

    fn update(&mut self, _display: &Display) {
        //display.queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&self.vertices));
    }

    fn render(&mut self, display: &mut Display, simulation: &Simulation) {

        let frame = display
            .swap_chain
            .get_current_frame().unwrap()
            .output;
        let mut encoder = display.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        let n_vertices = self.vertex_buffer.update(&mut encoder, &display, &simulation);


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
            render_pass.set_vertex_buffer(0, self.vertex_buffer.buf.slice(..));
            render_pass.set_bind_group(0, &self.bind_group, &[]);
            render_pass.draw(0..n_vertices, 0..1);
        }
    
        // Submit will accept anything that implements IntoIter
        // Submits the command buffer
        display.queue.submit(std::iter::once(encoder.finish()));

        // Recall all the used buffers
        display.spawner.spawn_local(self.vertex_buffer.belt.recall());
    }
}