use egui::{style::Margin, Color32, Frame};
use egui_wgpu::renderer::{Renderer, ScreenDescriptor};
use egui_winit::State;
use wgpu::TextureView;
use winit::event_loop::EventLoop;

use crate::{
    rendering::{
        gui_apps::{GridApp, PerfApp, StatApp},
        sim_renderer::SimRenderer,
        Display,
    },
    simulation::Simulation,
};

pub struct GUIRenderer {
    state: State,
    pub context: egui::Context,
    pub rpass: Renderer,
    debug: StatApp,
    grid: GridApp,
    perf: PerfApp,
}

impl GUIRenderer {
    pub fn new(display: &Display, _simulation: &mut Simulation, event_loop: &EventLoop<()>) -> Self {
        let state = egui_winit::State::new(event_loop);
        let context = egui::Context::default();
        context.set_pixels_per_point(display.window.scale_factor() as f32);

        let egui_rpass = Renderer::new(&display.device, display.surface_config.format, None, 1);

        Self {
            context,
            state,
            rpass: egui_rpass,
            debug: StatApp,
            grid: GridApp::default(),
            perf: PerfApp,
        }
    }

    pub fn pre_render(&mut self, display: &Display) {
        let input = self.state.take_egui_input(&display.window);
        self.context.begin_frame(input);
    }

    pub fn render(&mut self, display: &Display, simulation: &mut Simulation, sim_renderer: &mut SimRenderer) {
        let lpanel = egui::SidePanel::left("Debug Info");
        let rpanel = egui::SidePanel::right("Debug Info");
        let cpanel = egui::CentralPanel::default().frame(
            Frame::dark_canvas(&self.context.style())
                .fill(Color32::TRANSPARENT)
                .inner_margin(Margin::same(0.0)),
        );

        lpanel.show(&self.context, |ui| {
            self.debug.update(ui, display, simulation);
        });

        rpanel.show(&self.context, |ui| {
            self.perf.update(ui, display, simulation);
        });

        cpanel.show(&self.context, |ui| {
            self.grid.update(ui, display, simulation);
            sim_renderer.render(display, simulation, &mut self.rpass, ui)
        });
    }

    pub fn post_render(&mut self, display: &Display, view: &TextureView) {
        // End the UI frame. We could now handle the output and draw the UI with the backend.
        let output = self.context.end_frame();
        let paint_jobs = self.context.tessellate(output.shapes);

        let mut encoder = display.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("GUI Encoder"),
        });

        // Upload all resources for the GPU.
        let screen_descriptor = ScreenDescriptor {
            size_in_pixels: [display.surface_config.width, display.surface_config.height],
            pixels_per_point: display.window.scale_factor() as f32,
        };

        for (id, image_delta) in &output.textures_delta.set {
            self.rpass
                .update_texture(&display.device, &display.queue, *id, image_delta);
        }
        for id in &output.textures_delta.free {
            self.rpass.free_texture(id);
        }
        self.rpass.update_buffers(
            &display.device,
            &display.queue,
            &mut encoder,
            &paint_jobs,
            &screen_descriptor,
        );
        {
            // Set up render pass and associate the render pipeline we made
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            self.rpass.render(&mut render_pass, &paint_jobs, &screen_descriptor);
        }

        display.queue.submit(std::iter::once(encoder.finish()));
    }
}
