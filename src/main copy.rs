mod imgui_wrapper;
mod camera;

use core::f32;

use crate::imgui_wrapper::ImGuiWrapper;
use camera::Camera;
use ggez::event::{self, EventHandler, KeyCode, KeyMods, MouseButton};
use ggez::graphics::{self, Color, DrawParam, Mesh};
use ggez::{Context, GameResult, timer, conf};
use glam::Vec2 as Point2;

struct MainState {
    pos_x: f32,
    imgui_wrapper: ImGuiWrapper,
    hidpi_factor: f32,
    circle: Mesh,
    cam: Camera,
    pos: Point2
}

impl MainState {
    fn new(mut ctx: &mut Context, hidpi_factor: f32) -> GameResult<MainState> {
        let imgui_wrapper = ImGuiWrapper::new(&mut ctx);
        let (width, height) = ggez::graphics::size(ctx);

        let s = MainState {
            pos_x: 0.0,
            imgui_wrapper,
            hidpi_factor,
            circle: graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                Point2::ZERO,
                10.0,
                1.0,
                Color::WHITE,
            )?,
            cam: Camera::new(width as u32, height as u32, width, height),
            pos: Point2::new(0.0, 0.0)
        };
        Ok(s)
    }
}

impl EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let (width, _height) = ggez::graphics::size(ctx);
        self.pos_x = (self.pos_x + 100.0 * timer::delta(ctx).as_secs_f32()) % width;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::BLACK);

        graphics::draw(
            ctx, 
            &self.circle,
            DrawParam::default().dest(self.cam.world_to_screen_coords(self.pos))
        )?;

        self.imgui_wrapper.render(ctx, self.hidpi_factor);

        graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        self.imgui_wrapper.update_mouse_pos(x, y);
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        self.imgui_wrapper.update_mouse_down(button);
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, _x: f32, _y: f32) {
        self.imgui_wrapper.update_mouse_up(button);
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        keymods: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            event::KeyCode::Up => self.cam.move_by(Point2::new(0.0, 10.)),
            event::KeyCode::Left => self.cam.move_by(Point2::new(-10., 0.)),
            event::KeyCode::Down => self.cam.move_by(Point2::new(0.0, -10.)),
            event::KeyCode::Right => self.cam.move_by(Point2::new(10., 0.0)),
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
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, x: f32, y: f32) {
        self.imgui_wrapper.update_scroll(x, y);
    }
}

pub fn main() -> ggez::GameResult {
    let cb = ggez::ContextBuilder::new("super_simple with imgui", "ggez")
        .window_setup(
            conf::WindowSetup::default()
                .title("super_simple with imgui")
                .vsync(false),
        )
        .window_mode(
            conf::WindowMode::default().resizable(true), /*.dimensions(750.0, 500.0)*/
        );
    let (mut ctx, event_loop) = cb.build()?;

    let hidpi_factor = event_loop
        .primary_monitor().unwrap()
        .scale_factor() as f32;
    println!("main hidpi_factor = {}", hidpi_factor);

    let state = MainState::new(&mut ctx, hidpi_factor)?;

    event::run(ctx, event_loop, state)
}
