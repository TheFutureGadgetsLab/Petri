use super::imgui_wrapper::ImGuiWrapper;
use super::camera::*;
use ggez::event::{self, EventHandler, KeyCode, KeyMods, MouseButton};
use ggez::graphics::{self, Color, Mesh};
use ggez::{Context, GameResult, timer};
use glam::Vec2 as Vec2;

pub struct MainState {
    pub imgui_wrapper: ImGuiWrapper,
    pub hidpi_factor: f32,
    pub circle: Mesh,
    pub cam: Camera,
    pub click: bool
}

impl MainState {
    pub fn new(mut ctx: &mut Context, hidpi_factor: f32) -> GameResult<MainState> {
        let imgui_wrapper = ImGuiWrapper::new(&mut ctx);
        let win_size: Vec2 = ggez::graphics::size(ctx).into();

        let s = MainState {
            imgui_wrapper,
            hidpi_factor,
            circle: graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                Vec2::ZERO,
                2.0,
                1.0,
                Color::WHITE,
            )?,
            cam: Camera::new(win_size, win_size),
            click: false
        };
        Ok(s)
    }
}

impl EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        //let (width, _height) = ggez::graphics::size(ctx);
        //self.pos_x = (self.pos_x + 100.0 * timer::delta(ctx).as_secs_f32()) % width;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::BLACK);
        let w: f32 = 300.0;
        let h: f32 = 300.0;
        let t = timer::time_since_start(ctx).as_secs_f32();

        for i in 1..1000u32 {
            let i = i as f32;
            // let f = i as i32;
            let pos = Vec2::new(
                ((i * 0.6 + t).sin() * 0.5 + 0.5) * w,
                ((i + (16. + t * 0.5)).cos() * 0.5 + 0.5) * h
            );
            //let color = Color::from_rgb(f as u8, (f + 50) as u8, (f / 2 + 178) as u8);
            //let scale = ((i * 4.5 + t * 0.34).sin() * 0.5 + 0.5) * 4.0;
            self.circle.draw_camera(&self.cam, ctx, pos, 0.0)?;
        }

        self.imgui_wrapper.render(ctx, self.hidpi_factor);

        graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        self.imgui_wrapper.update_mouse_pos(x, y);
        if self.click {
            self.cam.move_by(glam::vec2(-_dx * self.cam.zoom, _dy * self.cam.zoom));
        }
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        self.imgui_wrapper.update_mouse_down(button);
        self.click = true;
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, _x: f32, _y: f32) {
        self.imgui_wrapper.update_mouse_up(button);
        self.click = false;
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        keymods: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            event::KeyCode::Up => self.cam.move_by(Vec2::new(0.0, 10.)),
            event::KeyCode::Left => self.cam.move_by(Vec2::new(-10., 0.)),
            event::KeyCode::Down => self.cam.move_by(Vec2::new(0.0, -10.)),
            event::KeyCode::Right => self.cam.move_by(Vec2::new(10., 0.0)),
            _ => (),
        };

        self.imgui_wrapper.update_key_down(keycode, keymods);
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, keymods: KeyMods) {
        self.imgui_wrapper.update_key_up(keycode, keymods);
    }

    fn text_input_event(&mut self, _ctx: &mut Context, val: char) {
        self.imgui_wrapper.update_text(val);
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        graphics::set_screen_coordinates(ctx, graphics::Rect::new(0.0, 0.0, width, height))
            .unwrap();
        self.cam.screen_size.x = width;
        self.cam.screen_size.y = height;
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, x: f32, y: f32) {
        self.imgui_wrapper.update_scroll(x, y);
        if y != 0.0 {
            self.cam.zoom *= 1.0 - y.signum() * 0.1;
        }
    }
}