use super::gui_renderer::GUIRenderer;
use super::sim_renderer::SimRenderer;
use ggez::event::{EventHandler, KeyCode, KeyMods, MouseButton, EventLoop};
use ggez::{Context, GameResult, graphics};

pub struct Renderer {
    pub gui: GUIRenderer,
    pub sim: SimRenderer,
    pub hidpi_factor: f32,
}

impl Renderer {
    pub fn new(mut ctx: &mut Context, event_loop: &EventLoop<()>) -> GameResult<Renderer> {
        let hidpi_factor = event_loop
            .primary_monitor().unwrap()
            .scale_factor() as f32;

        let gui_rend = GUIRenderer::new(&mut ctx);
        let sim_rend = SimRenderer::new(ctx)?;

        let rend = Renderer {
            gui: gui_rend,
            sim: sim_rend,
            hidpi_factor
        };
        Ok(rend)
    }
}

impl EventHandler<ggez::GameError> for Renderer {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.sim.draw(ctx)?;
        self.gui.draw(ctx, self.hidpi_factor);
        graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32, dx: f32, dy: f32) {
        self.sim.mouse_motion_event(ctx, x, y, dx, dy);
        self.gui.update_mouse_pos(x, y);
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32,
    ) {
        self.gui.update_mouse_down(button);
        self.sim.mouse_button_down_event(ctx, button, x, y)
    }

    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        self.sim.mouse_button_up_event(ctx, button, x, y);
        self.gui.update_mouse_up(button);
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, keymods: KeyMods, repeat: bool) {
        self.sim.key_down_event(ctx, keycode, keymods, repeat);
        self.gui.update_key_down(keycode, keymods);
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, keymods: KeyMods) {
        self.gui.update_key_up(keycode, keymods);
    }

    fn text_input_event(&mut self, _ctx: &mut Context, val: char) {
        self.gui.update_text(val);
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        self.sim.resize_event(ctx, width, height);
    }

    fn mouse_wheel_event(&mut self, ctx: &mut Context, x: f32, y: f32) {
        self.gui.update_scroll(x, y);
        self.sim.mouse_wheel_event(ctx, x, y);
    }
}