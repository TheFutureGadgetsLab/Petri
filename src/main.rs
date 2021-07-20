mod imgui_wrapper;

use crate::imgui_wrapper::ImGuiWrapper;
use ggez::event::{self, EventHandler, KeyCode, KeyMods, MouseButton};
use ggez::graphics::{self, Color, DrawParam, Mesh, Transform};
use ggez::{Context, GameResult, timer, conf};
use glam::Vec2 as Point2;

struct MainState {
    pos_x: f32,
    imgui_wrapper: ImGuiWrapper,
    hidpi_factor: f32,
    circle: Mesh,
}

impl MainState {
    fn new(mut ctx: &mut Context, hidpi_factor: f32) -> GameResult<MainState> {
        let imgui_wrapper = ImGuiWrapper::new(&mut ctx);
        let s = MainState {
            pos_x: 0.0,
            imgui_wrapper,
            hidpi_factor,
            circle: graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                Point2::ZERO,
                2.0,
                1.0,
                Color::WHITE,
            )?
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
        let (w, h) = ggez::graphics::size(ctx);
        let t = timer::time_since_start(ctx).as_secs_f32();

        for i in 1..1000u32 {
            let i = i as f32;
            let f = i as i32;
            let pos = [
                ((i * 0.6 + t).sin() * 0.5 + 0.5) * w,
                ((i + (16. + t * 0.5)).cos() * 0.5 + 0.5) * h];
            let color = Color::from_rgb(f as u8, (f + 50) as u8, (f / 2 + 178) as u8);
            let scale = ((i * 4.5 + t * 0.34).sin() * 0.5 + 0.5) * 4.0;
            graphics::draw(ctx, &self.circle,
                DrawParam::default().dest(pos).scale([scale, scale]).color(color))?;
        }

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
