use bevy::{
    app::prelude::*,
    asset::{Assets, HandleUntyped},
    core::FloatOrd,
    core_pipeline::Transparent2d,
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
use shaderc::{CompileOptions, ShaderKind};
use wgpu::{BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, MultisampleState, PrimitiveState};

pub const SHADER_VERT_HANDLE: HandleUntyped = HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 3032357527543835453);
pub const SHADER_VERT_SRC: &str = include_str!("tri.vert");

pub const SHADER_FRAG_HANDLE: HandleUntyped = HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 3032357527543835452);
pub const SHADER_FRAG_SRC: &str = include_str!("tri.frag");

pub struct CellRenderPlugin;

impl Plugin for CellRenderPlugin {
    fn build(&self, app: &mut App) {
        let shader_vert = Shader::from_spirv(compile_shader(ShaderKind::Vertex, SHADER_VERT_SRC));
        let shader_frag = Shader::from_spirv(compile_shader(ShaderKind::Fragment, SHADER_FRAG_SRC));

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
            .init_resource::<SpecializedPipelines<CellPipeline>>();

        let draw_cell = DrawCells::new(&mut render_app.world);
        render_app
            .world
            .get_resource::<DrawFunctions<Transparent2d>>()
            .unwrap()
            .write()
            .add(draw_cell);
    }
}

struct CellPipeline {
    view_layout: BindGroupLayout,
    view_bind_group: Option<BindGroup>,
    vertices: BufferVec<VertexCell>,
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

        Self {
            view_layout,
            view_bind_group: None,
            vertices: BufferVec::default(),
        }
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
                    attributes: wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x4, 2 => Float32].to_vec(),
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
            depth_stencil: None,
            layout: Some(vec![self.view_layout.clone()]),
            multisample: MultisampleState {
                count: 4,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            primitive: PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
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
    let mut cellbuf = render_world.get_resource_mut::<CellPipeline>().unwrap();
    cellbuf.vertices.clear();

    query.for_each(|trans| {
        cellbuf.vertices.push(VertexCell {
            position: trans.translation.into(),
            color: Vec4::new(0.0, 0.0, 1.0, 1.0).into(),
            size: trans.scale.x,
        });
    });
}

fn prepare_cells(
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
    mut commands: Commands,
    mut pipeline: ResMut<CellPipeline>,
) {
    pipeline.vertices.write_buffer(&render_device, &render_queue);
    commands.spawn_bundle((DummyDrawSentinel,));
}

fn queue_particles(
    draw_functions: Res<DrawFunctions<Transparent2d>>,
    mut views: Query<&mut RenderPhase<Transparent2d>>,
    render_device: Res<RenderDevice>,
    view_uniforms: Res<ViewUniforms>,
    mut cell_pipeline: ResMut<CellPipeline>,
    mut pipelines: ResMut<SpecializedPipelines<CellPipeline>>,
    mut pipeline_cache: ResMut<RenderPipelineCache>,
    cell_batches: Query<(Entity, &DummyDrawSentinel)>,
) {
    if view_uniforms.uniforms.is_empty() {
        return;
    }

    let layout = cell_pipeline.view_layout.clone();
    if let Some(view_bindings) = view_uniforms.uniforms.binding() {
        cell_pipeline.view_bind_group.get_or_insert_with(|| {
            render_device.create_bind_group(&BindGroupDescriptor {
                entries: &[BindGroupEntry {
                    binding: 0,
                    resource: view_bindings,
                }],
                label: Some("particle_view_bind_group".into()),
                layout: &layout,
            })
        });
    }

    let draw_particle_function = draw_functions.read().get_id::<DrawCells>().unwrap();
    for mut transparent_phase in views.iter_mut() {
        let (entity, _) = cell_batches.get_single().unwrap();
        transparent_phase.add(Transparent2d {
            sort_key: FloatOrd(0.0),
            entity: entity,
            pipeline: pipelines.specialize(&mut pipeline_cache, &cell_pipeline, CellPipelineKey),
            draw_function: draw_particle_function,
            batch_range: None,
        });
    }
}

struct DrawCells {
    params: SystemState<(
        SRes<CellPipeline>,
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

impl Draw<Transparent2d> for DrawCells {
    fn draw<'w>(&mut self, world: &'w World, pass: &mut TrackedRenderPass<'w>, view: Entity, item: &Transparent2d) {
        let (cell_pipeline, pipelines, views) = self.params.get(world);
        let n = cell_pipeline.vertices.len() as u32;

        let view_uniform = views.get(view).unwrap();
        let cellbuf = cell_pipeline.into_inner();

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

fn compile_shader(shader_kind: shaderc::ShaderKind, source_text: &str) -> Vec<u8> {
    let mut options = CompileOptions::new().unwrap();
    options.set_optimization_level(shaderc::OptimizationLevel::Performance);

    let mut compiler = shaderc::Compiler::new().unwrap();
    let comp = compiler.compile_into_spirv(source_text, shader_kind, "shader", "main", Some(&options));
    let comp = match comp {
        Ok(vert_comp) => vert_comp,
        Err(error) => {
            error!("Failed to compile shader");
            println!("{}", error.to_string());
            panic!();
        }
    };
    let source = comp.as_binary_u8().to_vec();

    source
}
