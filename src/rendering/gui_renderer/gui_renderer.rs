use imgui::*;
use imgui_wgpu::{Renderer, RendererConfig};
use wgpu::{CommandEncoder, RenderPass};
use std::time::Instant;
use crate::{
    rendering::{
        framework::{
            PetriEventLoop, Display
        }, 
    },
    simulation::Simulation
};

pub struct GUIRenderer {
    renderer: Renderer,
    imgui: imgui::Context,
    platform: imgui_winit_support::WinitPlatform,
    last_frame: Instant,
}

impl PetriEventLoop for GUIRenderer {
    fn init(display: &Display) -> GUIRenderer {
        let mut imgui = imgui::Context::create();
        let mut platform = imgui_winit_support::WinitPlatform::init(&mut imgui);
        platform.attach_window(
            imgui.io_mut(),
            &display.window,
            imgui_winit_support::HiDpiMode::Default,
        );
        imgui.set_ini_filename(None);

        let hidpi_factor = display.window.scale_factor();
        let font_size = (13.0 * hidpi_factor) as f32;
        imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

        imgui.fonts().add_font(&[FontSource::DefaultFontData {
            config: Some(imgui::FontConfig {
                oversample_h: 1,
                pixel_snap_h: true,
                size_pixels: font_size,
                ..Default::default()
            }),
        }]);

        let renderer_config = RendererConfig {
            texture_format: display.sc_desc.format,
            ..Default::default()
        };

        let renderer = Renderer::new(&mut imgui, &display.device, &display.queue, renderer_config);

        GUIRenderer {
            renderer: renderer,
            imgui: imgui,
            platform: platform,
            last_frame: Instant::now(),
        }
    }

    fn process_mouse(&mut self, _dx: f64, _dy: f64) {
    }

    fn resize(&mut self, _display: &Display) {
    }

    fn update(&mut self, _display: &Display) {
    }

    fn render_setup(&mut self, _display: &Display, _encoder: &mut CommandEncoder, _simulation: &Simulation) {
        
    }

    fn render<'b>(&'b mut self, display: &Display, render_pass: &mut RenderPass<'b>, _simulation: &Simulation) {
        let delta_s = self.last_frame.elapsed();
        let now = Instant::now();
        self.imgui.io_mut().update_delta_time(now - self.last_frame);
        self.last_frame = now;

        self.platform
            .prepare_frame(self.imgui.io_mut(), &display.window)
            .expect("Failed to prepare frame");
        let ui = self.imgui.frame();
        {
            let window = imgui::Window::new(im_str!("Hello world"));
            window
                .size([300.0, 100.0], Condition::FirstUseEver)
                .build(&ui, || {
                    ui.text(im_str!("Hello world!"));
                    ui.text(im_str!("This...is...imgui-rs on WGPU!"));
                    ui.separator();
                    let mouse_pos = ui.io().mouse_pos;
                    ui.text(im_str!(
                        "Mouse Position: ({:.1},{:.1})",
                        mouse_pos[0],
                        mouse_pos[1]
                    ));
                });

            let window = imgui::Window::new(im_str!("Hello too"));
            window
                .size([400.0, 200.0], Condition::FirstUseEver)
                .position([400.0, 200.0], Condition::FirstUseEver)
                .build(&ui, || {
                    ui.text(im_str!("Frametime: {:?}", delta_s));
                });

            ui.show_demo_window(&mut true);
        }
        self.platform.prepare_render(&ui, &display.window);

        self.renderer
            .render(ui.render(), &display.queue, &display.device, render_pass)
            .expect("Rendering failed");
    }

    fn render_end(&mut self, _display: &Display) {
        
    }
}