use super::camera::*;
use ggez::event::{self, KeyCode, KeyMods, MouseButton};
use ggez::graphics::{self, Color, MeshBatch};
use ggez::{Context, GameResult, timer};
use glam::Vec2 as Vec2;
use rayon::prelude::*;

pub const vertex_shader_src: &[u8] = br"
#version 150 core

in float a_SpawnTime;
in float a_LifeTime;

in vec2 a_Pos;
in vec2 a_Vel;
in float a_Angle;
in float a_AngularVel;
in float a_Friction;

in vec3 a_Color;
in float a_AlphaExp;

in vec2 a_Size;

out VertexData {
    float spawn_time;
    float life_time;
    vec4 color;
    vec2 size;
} VertexOut;

uniform Globals {
    mat4 u_Transform;
    float u_Time;
};

void main() {
    // TODO: Not sure if it is besser to calculate derived particle properties
    //       in vertex or geometry shader.

    float delta = u_Time - a_SpawnTime;
    float percent = delta / a_LifeTime;

    vec2 pos = a_Pos 
        + a_Vel * delta
        - 0.5 * a_Friction * delta * delta * normalize(a_Vel);
    gl_Position = vec4(pos, 0, 1);

    VertexOut.color = vec4(a_Color, 1.0 - pow(percent, a_AlphaExp));
    VertexOut.spawn_time = a_SpawnTime;
    VertexOut.life_time = a_LifeTime;
    VertexOut.size = a_Size;
}";

pub const pixel_shader_src: &[u8] = br"
#version 150 core

in VertexData {
    vec4 color;
    vec2 uv;
} VertexIn;

out vec4 Target0;

void main() {
    float alpha = max(1 - dot(VertexIn.uv, VertexIn.uv), 0);
    Target0 = vec4(VertexIn.color.xyz, VertexIn.color.w * alpha);
}";


pub struct SimRenderer {
    pub cam: Camera,
    pub click: bool,
    circs: Vec<Circ>,
    mesh_batch: MeshBatch,
    mouse_pos: Vec2
}

struct Circ {
    pub pos: Vec2,
    pub idx: f32,
    pub color: Color,
    pub scale: f32
}

impl SimRenderer {
    pub fn new(ctx: &mut Context) -> GameResult<SimRenderer> {
        let mut circs: Vec<Circ> = Vec::new();
        for _i in 1..50_000u32 {
            circs.push( Circ { pos: Vec2::ZERO, idx: _i as f32, color: Color::WHITE, scale: 1.0 });
        }

        let mesh_batch = graphics::MeshBatch::new(
            graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(), 
                Vec2::ZERO,
                2.0,
                0.1,
                Color::WHITE,
        )?)?;


        let s = SimRenderer {
            cam: Camera::new(),
            click: false,
            circs,
            mesh_batch,
            mouse_pos: Vec2::ZERO
        };
        Ok(s)
    }

    pub fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let w: f32 = 800.0;
        let h: f32 = 800.0;
        let t = timer::time_since_start(ctx).as_secs_f32() * 0.01;

        self.circs.par_chunks_mut(128).for_each(|circs| {
            for circ in circs.iter_mut() {
                let i = circ.idx;
                circ.pos.x = ((i * 0.0662 + t * 0.9).sin() * 0.5 + 0.5) * w;
                circ.pos.y = ((i * 0.1123 + (16. + t * 0.5)).cos() * 0.5 + 0.5) * h;
                circ.color.r = (t * 0.2 + 6.6 + i * 0.25).sin() * 0.5 + 0.5;
                circ.color.g = (t * 0.1 + 1.6 + i * 0.60).sin() * 0.5 + 0.5;
                circ.color.b = (t * 0.3 + 9.1 + i * 0.10).sin() * 0.5 + 0.5;
                circ.scale = 0.1 + 2.0 * ((t * 0.8 + 1.1 + i * 0.70).sin() * 0.5 + 0.5);
            }
        });
        Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.mesh_batch.clear();

        for circ in self.circs.iter() {
            let p = graphics::DrawParam::new()
                    .dest(circ.pos)
                    .color(circ.color)
                    .scale([circ.scale, circ.scale]);
            self.mesh_batch.add(p);
        }

        self.mesh_batch.draw(ctx, self.cam.transform)
    }

    pub fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, dx: f32, dy: f32) {
        self.mouse_pos = [x, y].into();
        if self.click {
            self.cam.move_by([dx, dy].into());
        }
    }

    pub fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        self.click = true;
    }

    pub fn mouse_button_up_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32) {
        self.click = false;
    }

    pub fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            event::KeyCode::Up => self.cam.move_by(Vec2::new(0.0, 10.)),
            event::KeyCode::Left => self.cam.move_by(Vec2::new(-10., 0.)),
            event::KeyCode::Down => self.cam.move_by(Vec2::new(0.0, -10.)),
            event::KeyCode::Right => self.cam.move_by(Vec2::new(10., 0.0)),
            _ => (),
        };
    }

    pub fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        graphics::set_screen_coordinates(ctx, graphics::Rect::new(0.0, 0.0, width, height))
            .unwrap();
    }

    pub fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: f32, y: f32) {
        if y != 0.0 {
            self.cam.zoom_by(1.0 + y.signum() * 0.1, self.mouse_pos);
        }
    }
}

