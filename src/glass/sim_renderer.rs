use super::camera::*;
use ggez::event::{self, KeyCode, KeyMods, MouseButton};
use ggez::graphics::{self, Color, MeshBatch};
use ggez::{Context, GameResult, timer};
use glam::Vec2 as Vec2;
use rayon::prelude::*;

pub struct SimRenderer {
    pub cam: Camera,
    pub click: bool,
    circs: Vec<Circ>,
    mesh_batch: MeshBatch
}

struct Circ {
    pub pos: Vec2,
    pub idx: f32
}

impl SimRenderer {
    pub fn new(ctx: &mut Context) -> GameResult<SimRenderer> {
        let mut circs: Vec<Circ> = Vec::new();
        for i in 1..100_000u32 {
            circs.push( Circ { pos: Vec2::ZERO , idx: i as f32});
        }

        let mesh_batch = graphics::MeshBatch::new(
            graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(), 
                Vec2::ZERO,
                2.0,
                2.0,
                Color::WHITE,
        )?)?;


        let s = SimRenderer {
            cam: Camera::new(),
            click: false,
            circs,
            mesh_batch: mesh_batch
        };
        Ok(s)
    }

    pub fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let w: f32 = 300.0;
        let h: f32 = 300.0;
        let t = timer::time_since_start(ctx).as_secs_f32();

        self.circs.par_chunks_mut(128).for_each(|circs| {
            for circ in circs.iter_mut() {
                circ.pos = Vec2::new(
                    ((circ.idx * 0.6 + t).sin() * 0.5 + 0.5) * w,
                    ((circ.idx + (16. + t * 0.5)).cos() * 0.5 + 0.5) * h,
                );
            }
        });
        Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.mesh_batch.clear();

        for circ in self.circs.iter() {
            let p = graphics::DrawParam::new()
                    .dest(circ.pos);
            self.mesh_batch.add(p);
        }

        self.mesh_batch.draw(ctx, self.cam.transform)
    }

    pub fn mouse_motion_event(&mut self, _ctx: &mut Context, _x: f32,_y: f32, _dx: f32, _dy: f32) {
        if self.click {
            self.cam.move_by([-_dx, _dy].into());
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
            self.cam.zoom_by(1.0 - y.signum() * 0.1);
        }
    }
}

