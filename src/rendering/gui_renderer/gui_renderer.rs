use imgui::*;
use imgui_wgpu::{Renderer, RendererConfig};
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

    fn render(&mut self, display: &mut Display, _simulation: &Simulation) {
        let delta_s = self.last_frame.elapsed();
        let now = Instant::now();
        self.imgui.io_mut().update_delta_time(now - self.last_frame);
        self.last_frame = now;

        let frame = match display.swap_chain.get_current_frame() {
            Ok(frame) => frame,
            Err(e) => {
                eprintln!("dropped frame: {:?}", e);
                return;
            }
        };
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

        let mut encoder: wgpu::CommandEncoder =
            display.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        self.platform.prepare_render(&ui, &display.window);

        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: &frame.output.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 0.0,
                    }),
                    store: true,
                }
            }],
            depth_stencil_attachment: None,
        });

        self.renderer
            .render(ui.render(), &display.queue, &display.device, &mut rpass)
            .expect("Rendering failed");

        drop(rpass);
        display.queue.submit(Some(encoder.finish()));
    }
}