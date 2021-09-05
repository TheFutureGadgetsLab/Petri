use std::time::Instant;

use legion::*;

use crate::{
    rendering::Display,
    simulation::{Color, RigidCircle, Simulation},
    timing::TIMING_DATABASE,
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
        }
    }

    pub fn update(&mut self, display: &Display, simulation: &Simulation) -> u32 {
        let start = Instant::now();

        let vertices: Vec<Vertex> = <(&RigidCircle, &Color)>::query()
            .iter(&simulation.world)
            .map(|(circ, color)| Vertex::new(circ, &color.val))
            .collect();

        display
            .queue
            .write_buffer(&self.buf, 0, bytemuck::cast_slice(&vertices));

        TIMING_DATABASE
            .write()
            .sim_render
            .vertex_buffer_update
            .update(Instant::now() - start);

        vertices.len() as u32
    }
}
