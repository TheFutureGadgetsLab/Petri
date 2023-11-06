use bytemuck;
use egui_wgpu::Renderer;
use naga;
use wgpu::ShaderModuleDescriptor;

use crate::{
    rendering::{camera::CameraUniform, Display, Vertex, VertexBuffer},
    simulation::Simulation,
    timing::timer::time_func,
};

pub struct SimRenderer;

impl SimRenderer {
    pub fn new(display: &Display, _simulation: &mut Simulation, renderer: &mut egui_wgpu::Renderer) -> Self {
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
                    visibility: wgpu::ShaderStages::VERTEX,
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

        let shader_frag = &display.device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Fragment Shader"),
            source: wgpu::ShaderSource::Glsl {
                shader: std::borrow::Cow::Borrowed(include_str!("./shaders/particles.frag")),
                stage: naga::ShaderStage::Fragment,
                defines: Default::default(),
            },
        });

        let shader_vert = &display.device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Vertex Shader"),
            source: wgpu::ShaderSource::Glsl {
                shader: std::borrow::Cow::Borrowed(include_str!("./shaders/particles.vert")),
                stage: naga::ShaderStage::Vertex,
                defines: Default::default(),
            },
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
            multiview: None,
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
                targets: &[Some(wgpu::ColorTargetState {
                    format: display.surface_config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::PointList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Point,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
        });

        let resources = SimRenderResources::new(render_pipeline, bind_group, uniforms_ubo, display);

        renderer.callback_resources.insert(resources);

        Self {}
    }

    pub fn render(&mut self, display: &Display, simulation: &mut Simulation, ctx: &mut Renderer, ui: &mut egui::Ui) {
        time_func!("render.step");
        let cam_uniform = CameraUniform::from(&display.cam);

        let resources = ctx.callback_resources.get_mut::<SimRenderResources>().unwrap();

        resources.vertex_buffer.update(simulation);
        resources.camera_uniform = cam_uniform;

        let rect = ui.min_rect();
        ui.painter()
            .add(egui_wgpu::Callback::new_paint_callback(rect, SimDrawCallback {}));
    }
}

struct SimDrawCallback {}

impl egui_wgpu::CallbackTrait for SimDrawCallback {
    fn prepare(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        _egui_encoder: &mut wgpu::CommandEncoder,
        resources: &mut egui_wgpu::CallbackResources,
    ) -> Vec<wgpu::CommandBuffer> {
        let resources: &mut SimRenderResources = resources.get_mut().unwrap();
        resources.prepare(device, queue);
        Vec::new()
    }

    fn paint<'a>(
        &self,
        _info: egui::PaintCallbackInfo,
        render_pass: &mut wgpu::RenderPass<'a>,
        resources: &'a egui_wgpu::CallbackResources,
    ) {
        let resources: &SimRenderResources = resources.get().unwrap();
        resources.paint(render_pass);
    }
}

struct SimRenderResources {
    pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
    uniform_buffer: wgpu::Buffer,

    vertex_buffer: VertexBuffer,
    camera_uniform: CameraUniform,
}

impl SimRenderResources {
    fn new(
        pipeline: wgpu::RenderPipeline,
        bind_group: wgpu::BindGroup,
        uniform_buffer: wgpu::Buffer,
        display: &Display,
    ) -> Self {
        Self {
            pipeline,
            bind_group,
            uniform_buffer,
            vertex_buffer: VertexBuffer::default(display),
            camera_uniform: CameraUniform::default(),
        }
    }

    fn prepare(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {
        self.vertex_buffer.write(queue, device);
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[self.camera_uniform]));
    }

    fn paint<'rp>(&'rp self, render_pass: &mut wgpu::RenderPass<'rp>) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.buf.slice(..));
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.draw(0..(self.vertex_buffer.cur_verticies.len() as u32), 0..1);
    }
}
