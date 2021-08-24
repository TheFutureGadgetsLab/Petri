use crate::{
    rendering::{Camera, Display},
    simulation::{RigidCircle, Simulation}
};
use legion::*;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, Default)]
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
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        VertexBuffer {
            buf: vertex_buffer,
            size: size,
        }
    }

    pub fn update(&mut self, display: &Display, simulation: &Simulation, cam: &Camera) -> u32 {
        let vertices: Vec<Vertex> = <&RigidCircle>::query().iter(&simulation.world).map(|circ| {
            let (pos, scale) = cam.transform(circ.pos, [circ.radius; 2].into());
            Vertex {position: pos.into(), size: scale.x, color: circ.color}
        }).collect();

        display.queue.write_buffer(&self.buf, 0, bytemuck::cast_slice(&vertices));

        vertices.len() as u32
    }
}