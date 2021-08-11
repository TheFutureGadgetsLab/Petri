use crate::simulation::{RigidCircle, Simulation};
use crate::rendering::{camera::Camera, framework::Display};
use legion::*;
use wgpu::CommandEncoder;
use wgpu::util::StagingBelt;

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
    vertices: Vec<Vertex>,
    pub belt: StagingBelt,
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
            size: size,
            vertices: vec![Vertex::default(); size],
            belt: StagingBelt::new(3_000_000),
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

    pub fn update(&mut self, display: &Display, encoder: &mut CommandEncoder, simulation: &Simulation, cam: &Camera) -> u32 {
        for (i, circ) in <&RigidCircle>::query().iter(&simulation.world).enumerate() {
            let (pos, scale) = cam.transform(circ.pos, [circ.radius; 2].into());
            self.vertices[i].position = pos.into();
            self.vertices[i].size = scale.x;
            self.vertices[i].color = circ.color;
        }

        if self.size < self.vertices.len() {
            self.reallocate(display, self.vertices.len());
        }

        let bufsize = (std::mem::size_of::<Vertex>() * self.size) as u64;

        self.belt.write_buffer(
            encoder, 
            &self.buf,
            0,
            wgpu::BufferSize::new(bufsize).unwrap(), 
            &display.device
        )
            .copy_from_slice(bytemuck::cast_slice(&self.vertices));

        self.belt.finish();

        self.vertices.len() as u32
    }
}