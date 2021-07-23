use super::camera::*;
use ggez::event::{self, KeyCode, KeyMods, MouseButton};
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult, timer};
use glam::Vec2 as Vec2;

pub struct SimRenderer {
    pub cam: Camera,
    pub click: bool,
    circs: Vec<Circ>,
}

struct Circ {
    pub pos: Vec2
}

impl SimRenderer {
    pub fn new(ctx: &mut Context) -> GameResult<SimRenderer> {
        let win_size: Vec2 = ggez::graphics::size(ctx).into();

        let mut circs: Vec<Circ> = Vec::new();
        for _i in 1..10000u32 {
            circs.push( Circ { pos: Vec2::ZERO });
        }

        let s = SimRenderer {
            cam: Camera::new(win_size, win_size),
            click: false,
            circs
        };
        Ok(s)
    }

    pub fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let w: f32 = 300.0;
        let h: f32 = 300.0;
        let t = timer::time_since_start(ctx).as_secs_f32();

        for (i, circ) in self.circs.iter_mut().enumerate() {
            let i = i as f32;
            circ.pos.x = ((i * 0.6 + t).sin() * 0.5 + 0.5) * w;
            circ.pos.y = ((i + (16. + t * 0.5)).cos() * 0.5 + 0.5) * h;
        }
        Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut mesh_batch = graphics::MeshBatch::new(
            graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(), 
                Vec2::ZERO,
                2.0,
                2.0,
                Color::WHITE,
        )?)?;

        for circ in self.circs.iter() {
            let p = graphics::DrawParam::new()
                    .dest(self.cam.world_to_screen_coords(circ.pos));
                mesh_batch.add(p);
        }

        mesh_batch.draw(ctx, graphics::DrawParam::default())
    }

    pub fn mouse_motion_event(&mut self, _ctx: &mut Context, _x: f32,_y: f32, _dx: f32, _dy: f32) {
        if self.click {
            self.cam.move_by(glam::vec2(-_dx * self.cam.zoom, _dy * self.cam.zoom));
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
        self.cam.screen_size.x = width;
        self.cam.screen_size.y = height;
    }

    pub fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: f32, y: f32) {
        if y != 0.0 {
            self.cam.zoom *= 1.0 - y.signum() * 0.1;
        }
    }
}

