use crate::simulation::{RigidCircle, Simulation};
use crate::rendering::framework::Display;
use legion::*;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 2],
    color: [f32; 4],
    size: f32
}
impl From<&RigidCircle> for Vertex {
    fn from(item: &RigidCircle) -> Vertex {
        Vertex {
            position: item.pos.into(),
            color: item.color,
            size: item.radius
        }
    }
}

pub struct VertexBuffer {
    pub buf: wgpu::Buffer,
    pub size: usize,
}

impl VertexBuffer {
    pub fn default(display: &Display) -> VertexBuffer {
        let size: usize = 3_000_000;
        let init_alloc = std::mem::size_of::<Vertex>() * size;

        let vertex_buffer = display.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Vertex buffer"),
            size: init_alloc as wgpu::BufferAddress,
            usage: wgpu::BufferUsage::VERTEX | wgpu::BufferUsage::COPY_DST,
            mapped_at_creation: false,
        });

        VertexBuffer {
            buf: vertex_buffer,
            size: size
        }
    }

    pub fn reallocate(&mut self, display: &Display, size: usize) {
        let init_alloc = std::mem::size_of::<Vertex>() * size;
        self.buf = display.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Vertex buffer"),
            size: init_alloc as wgpu::BufferAddress,
            usage: wgpu::BufferUsage::VERTEX | wgpu::BufferUsage::COPY_DST,
            mapped_at_creation: false,
        });
        self.size = size;
    }

    pub fn fill(&mut self, display: &Display, simulation: &Simulation) -> u32 {
        let vertices: Vec<Vertex> = <&RigidCircle>::query().iter(&simulation.world).map(|circ| circ.into()).collect();

        if self.size < vertices.len() {
            self.reallocate(display, vertices.len());
        }

        display.queue.write_buffer(&self.buf, 0, bytemuck::cast_slice(&vertices));

        vertices.len() as u32
    }
}