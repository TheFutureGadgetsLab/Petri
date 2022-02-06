use bevy::{
    app::prelude::*,
    asset::{Assets, HandleUntyped},
    core_pipeline::Transparent3d,
    ecs::{
        prelude::*,
        system::{lifetimeless::*, SystemState},
    },
    math::prelude::*,
    prelude::*,
    reflect::TypeUuid,
    render::{
        render_phase::{Draw, DrawFunctions, RenderPhase, TrackedRenderPass},
        render_resource::*,
        renderer::{RenderDevice, RenderQueue},
        texture::BevyDefault,
        view::{ViewUniform, ViewUniformOffset, ViewUniforms},
        RenderApp, RenderStage, RenderWorld,
    },
};
use shaderc::CompileOptions;
use wgpu::{BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, MultisampleState, PrimitiveState};

pub const SHADER_VERT_HANDLE: HandleUntyped = HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 3032357527543835453);
pub const SHADER_FRAG_HANDLE: HandleUntyped = HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 3032357527543835452);

pub struct CellRenderPlugin;

impl Plugin for CellRenderPlugin {
    fn build(&self, app: &mut App) {
        let (vert_source, frag_source) = compile_shaders();
        let shader_vert = Shader::from_spirv(vert_source);
        let shader_frag = Shader::from_spirv(frag_source);

        app.world
            .get_resource_mut::<Assets<Shader>>()
            .unwrap()
            .set_untracked(SHADER_VERT_HANDLE, shader_vert);
        app.world
            .get_resource_mut::<Assets<Shader>>()
            .unwrap()
            .set_untracked(SHADER_FRAG_HANDLE, shader_frag);

        let render_app = app.get_sub_app_mut(RenderApp).unwrap();
        render_app
            .add_system_to_stage(RenderStage::Extract, extract_cells)
            .add_system_to_stage(RenderStage::Prepare, prepare_cells)
            .add_system_to_stage(RenderStage::Queue, queue_particles)
            .init_resource::<CellPipeline>()
            .init_resource::<CellGPUBuf>()
            .init_resource::<SpecializedPipelines<CellPipeline>>();

        let draw_cell = DrawCells::new(&mut render_app.world);
        render_app
            .world
            .get_resource::<DrawFunctions<Transparent3d>>()
            .unwrap()
            .write()
            .add(draw_cell);
    }
}

struct CellPipeline {
    view_layout: BindGroupLayout,
}

impl FromWorld for CellPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.get_resource::<RenderDevice>().unwrap();

        // View uniform
        let view_layout = render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: true,
                    min_binding_size: BufferSize::new(std::mem::size_of::<ViewUniform>() as u64),
                },
                count: None,
            }],
            label: None,
        });

        Self { view_layout }
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct CellPipelineKey;

impl SpecializedPipeline for CellPipeline {
    type Key = CellPipelineKey;

    fn specialize(&self, _: Self::Key) -> RenderPipelineDescriptor {
        RenderPipelineDescriptor {
            label: Some("cell_render_pipeline".into()),
            vertex: VertexState {
                shader: SHADER_VERT_HANDLE.typed::<Shader>(),
                entry_point: "main".into(),
                buffers: vec![VertexBufferLayout {
                    array_stride: std::mem::size_of::<VertexCell>() as u64,
                    step_mode: VertexStepMode::Vertex,
                    attributes: wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x4, 2 => Float32].to_vec(),
                }],
                shader_defs: vec![],
            },
            fragment: Some(FragmentState {
                shader: SHADER_FRAG_HANDLE.typed::<Shader>(),
                shader_defs: vec![],
                entry_point: "main".into(),
                targets: vec![wgpu::ColorTargetState {
                    format: TextureFormat::bevy_default(),
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
            }),
            depth_stencil: Some(DepthStencilState {
                format: TextureFormat::Depth32Float,
                depth_write_enabled: false,
                depth_compare: CompareFunction::Never,
                stencil: StencilState {
                    front: StencilFaceState::IGNORE,
                    back: StencilFaceState::IGNORE,
                    read_mask: 0,
                    write_mask: 0,
                },
                bias: DepthBiasState {
                    constant: 0,
                    slope_scale: 0.0,
                    clamp: 0.0,
                },
            }),
            layout: Some(vec![self.view_layout.clone()]),
            multisample: MultisampleState::default(),
            primitive: PrimitiveState {
                topology: wgpu::PrimitiveTopology::PointList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Point,
                unclipped_depth: true,
                conservative: false,
            },
        }
    }
}

#[derive(Component, Default)]
pub struct CellMarker;

#[derive(Bundle, Default)]
pub struct CellBundle {
    pub marker: CellMarker,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, Default)]
pub struct VertexCell {
    position: [f32; 3],
    color: [f32; 4],
    size: f32,
}

fn extract_cells(mut render_world: ResMut<RenderWorld>, query: Query<&Transform, With<CellMarker>>) {
    let mut cellbuf = render_world.get_resource_mut::<CellGPUBuf>().unwrap();
    cellbuf.vertices.clear();

    for cell in query.iter() {
        cellbuf.vertices.push(VertexCell {
            position: cell.translation.into(),
            color: Vec4::new(0.0, 0.0, 1.0, 1.0).into(),
            size: cell.scale.x,
        });
    }
}

struct CellGPUBuf {
    view_bind_group: Option<BindGroup>,

    vertices: BufferVec<VertexCell>,
}

impl Default for CellGPUBuf {
    fn default() -> Self {
        CellGPUBuf {
            view_bind_group: None,
            vertices: BufferVec::default(),
        }
    }
}

fn prepare_cells(
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
    mut commands: Commands,
    mut cellbuf: ResMut<CellGPUBuf>,
) {
    cellbuf.vertices.write_buffer(&render_device, &render_queue);
    commands.spawn_bundle((DummyDrawSentinel,));
}

#[allow(clippy::too_many_arguments)]
fn queue_particles(
    draw_functions: Res<DrawFunctions<Transparent3d>>,
    mut views: Query<&mut RenderPhase<Transparent3d>>,
    render_device: Res<RenderDevice>,
    mut cellbuf: ResMut<CellGPUBuf>,
    view_uniforms: Res<ViewUniforms>,
    particle_pipeline: Res<CellPipeline>,
    mut pipelines: ResMut<SpecializedPipelines<CellPipeline>>,
    mut pipeline_cache: ResMut<RenderPipelineCache>,
    particle_batches: Query<(Entity, &DummyDrawSentinel)>,
) {
    if view_uniforms.uniforms.is_empty() {
        error!("View uniforms is empty!");
        return;
    }

    if let Some(view_bindings) = view_uniforms.uniforms.binding() {
        cellbuf.view_bind_group.get_or_insert_with(|| {
            render_device.create_bind_group(&BindGroupDescriptor {
                entries: &[BindGroupEntry {
                    binding: 0,
                    resource: view_bindings,
                }],
                label: Some("particle_view_bind_group".into()),
                layout: &particle_pipeline.view_layout,
            })
        });
    }

    let draw_particle_function = draw_functions.read().get_id::<DrawCells>().unwrap();
    for mut transparent_phase in views.iter_mut() {
        let (entity, _) = particle_batches.get_single().unwrap();
        transparent_phase.add(Transparent3d {
            distance: 0.0,
            pipeline: pipelines.specialize(&mut pipeline_cache, &particle_pipeline, CellPipelineKey),
            entity,
            draw_function: draw_particle_function,
        });
    }
}

struct DrawCells {
    params: SystemState<(
        SRes<CellGPUBuf>,
        SRes<RenderPipelineCache>,
        SQuery<Read<ViewUniformOffset>>,
    )>,
}

impl DrawCells {
    fn new(world: &mut World) -> Self {
        Self {
            params: SystemState::new(world),
        }
    }
}

impl Draw<Transparent3d> for DrawCells {
    fn draw<'w>(&mut self, world: &'w World, pass: &mut TrackedRenderPass<'w>, view: Entity, item: &Transparent3d) {
        let (cellbuf, pipelines, views) = self.params.get(world);
        let n = cellbuf.vertices.len() as u32;
        info!("Drawing {} vertices", n);

        let view_uniform = views.get(view).unwrap();
        let cellbuf = cellbuf.into_inner();

        if let Some(pipeline) = pipelines.into_inner().get(item.pipeline) {
            pass.set_render_pipeline(pipeline);
            pass.set_vertex_buffer(0, cellbuf.vertices.buffer().unwrap().slice(..));
            pass.set_bind_group(0, cellbuf.view_bind_group.as_ref().unwrap(), &[view_uniform.offset]);
            pass.draw(0..n, 0..1);
        }
    }
}

#[derive(Component)]
struct DummyDrawSentinel;

fn compile_shaders() -> (Vec<u8>, Vec<u8>) {
    info!("Compiling shaders");

    let mut options = CompileOptions::new().unwrap();
    options.set_optimization_level(shaderc::OptimizationLevel::Performance);

    let mut compiler = shaderc::Compiler::new().unwrap();
    let vert_comp = compiler.compile_into_spirv(
        include_str!("particle.vert"),
        shaderc::ShaderKind::Vertex,
        "particle.vert",
        "main",
        Some(&options),
    );
    let vert_comp = match vert_comp {
        Ok(vert_comp) => vert_comp,
        Err(error) => {
            error!("Failed to compile Vertex shader");
            println!("{}", error.to_string());
            panic!();
        }
    };
    let frag_comp = compiler.compile_into_spirv(
        include_str!("particle.frag"),
        shaderc::ShaderKind::Fragment,
        "particle.frag",
        "main",
        Some(&options),
    );
    let frag_comp = match frag_comp {
        Ok(frag_comp) => frag_comp,
        Err(error) => {
            error!("Failed to compile Fragment shader");
            println!("{}", error.to_string());
            panic!();
        }
    };

    let vert_source = vert_comp.as_binary_u8().to_vec();
    let frag_source = frag_comp.as_binary_u8().to_vec();

    info!("Compiled shaders");
    (vert_source, frag_source)
}
