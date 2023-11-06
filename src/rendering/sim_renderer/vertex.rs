use crate::{
    rendering::Display,
    simulation::{Color, RigidCircle, Simulation},
    timing::timer::time_func,
};

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, Default)]
pub struct Vertex {
    position: [f32; 2],
    color: [f32; 4],
    radius: f32,
}

impl Vertex {
    pub fn new(circ: &RigidCircle, color: &[f32; 4]) -> Self {
        Self {
            position: circ.pos.into(),
            color: *color,
            radius: circ.radius,
        }
    }
}

pub struct VertexBuffer {
    pub buf: wgpu::Buffer,
    pub cur_verticies: Vec<Vertex>,
    pub size: usize,
}

impl VertexBuffer {
    pub fn default(display: &Display) -> VertexBuffer {
        let size: usize = 3_000_000;
        let init_alloc = std::mem::size_of::<Vertex>() * size;

        let vertex_buffer = display.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Vertex buffer"),
            size: init_alloc as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        VertexBuffer {
            buf: vertex_buffer,
            size,
            cur_verticies: Vec::with_capacity(size),
        }
    }

    pub fn update(&mut self, simulation: &mut Simulation) {
        time_func!("render.vertex_update");

        let mut query = simulation.world.query::<(&RigidCircle, &Color)>();
        self.cur_verticies.clear();
        self.cur_verticies.extend(
            query
                .iter(&simulation.world)
                .map(|(circ, color)| Vertex::new(circ, &color.val)),
        );
    }

    pub fn write(&mut self, queue: &wgpu::Queue, device: &wgpu::Device) {
        let alloc_size = std::mem::size_of::<Vertex>() * self.cur_verticies.len();
        // reallocate if buffer is too small
        if alloc_size > self.buf.size() as usize {
            self.buf = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Vertex buffer"),
                size: alloc_size as wgpu::BufferAddress,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        }
        queue.write_buffer(&self.buf, 0, bytemuck::cast_slice(&self.cur_verticies));
    }
}
