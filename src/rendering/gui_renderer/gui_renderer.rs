use crate::{rendering::{
        framework::{
            PetriEventLoop, Display,
        }, 
    }, simulation::{Simulation}};
use std::{iter, sync::Arc};
use std::time::Instant;

use egui::{Align2, FontDefinitions};
use egui_wgpu_backend::{RenderPass, ScreenDescriptor};
use egui_winit_platform::{Platform, PlatformDescriptor};
use epi::*;


#[derive(Default)]
pub struct DummyRepaintSignal(bool);
impl epi::RepaintSignal for DummyRepaintSignal {
    fn request_repaint(&self) {
    }
}

pub struct GUIRenderer {
    platform: Platform,
    rpass: RenderPass,
    start_time: Instant,
    previous_frame_time: Option<f32>,
    signal: Arc<DummyRepaintSignal>
}

impl PetriEventLoop for GUIRenderer {
    fn init(display: &Display) -> GUIRenderer {
        let size = display.window.inner_size();
        // We use the egui_winit_platform crate as the platform.
        let platform = Platform::new(PlatformDescriptor {
            physical_width: size.width as u32,
            physical_height: size.height as u32,
            scale_factor: display.window.scale_factor(),
            font_definitions: FontDefinitions::default(),
            style: Default::default(),
        });

        
        let repaint_signal = std::sync::Arc::new(DummyRepaintSignal::default());

        // We use the egui_wgpu_backend crate as the render backend.
        let egui_rpass = RenderPass::new(&display.device, display.sc_desc.format, 1);
        GUIRenderer {
            platform: platform,
            rpass: egui_rpass,
            start_time: Instant::now(),
            previous_frame_time: None,
            signal: repaint_signal,
        }
    }

    fn handle_event<T>(&mut self, _display: &Display, event: &winit::event::Event<T>) {
        self.platform.handle_event(&event)        
    }

    fn update(&mut self, _display: &Display) {
        
    }

    fn render(&mut self, display: &Display, _simulation: &Simulation) {
        self.platform.update_time(self.start_time.elapsed().as_secs_f64());

        let output_frame = match display.swap_chain.get_current_frame() {
            Ok(frame) => frame,
            Err(_) => {
                // Dropped frame?
                return;
            }
        };

        // Begin to draw the UI frame.
        let egui_start = Instant::now();
        self.platform.begin_frame();
        let mut app_output = epi::backend::AppOutput::default();

        epi::backend::FrameBuilder {
            info: epi::IntegrationInfo {
                web_info: None,
                cpu_usage: self.previous_frame_time,
                seconds_since_midnight: None,
                native_pixels_per_point: Some(display.window.scale_factor() as _),
                prefer_dark_mode: None,
            },
            tex_allocator: &mut self.rpass,
            output: &mut app_output,
            repaint_signal: self.signal.clone(),
        }
        .build();

        // Draw stuff
        egui::Window::new("Debug Info")
            .resizable(false)
            .anchor(Align2::LEFT_TOP, [0., 0.])
            .show(&self.platform.context(), |ui| {
                ui.label(format!("time elapsed: {:.2}", self.start_time.elapsed().as_secs_f64()));
            });

        // End the UI frame. We could now handle the output and draw the UI with the backend.
        let (_output, paint_commands) = self.platform.end_frame();
        let paint_jobs = self.platform.context().tessellate(paint_commands);

        let frame_time = (Instant::now() - egui_start).as_secs_f64() as f32;
        self.previous_frame_time = Some(frame_time);

        let mut encoder = display.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("encoder"),
        });

        // Upload all resources for the GPU.
        let screen_descriptor = ScreenDescriptor {
            physical_width: display.sc_desc.width,
            physical_height: display.sc_desc.height,
            scale_factor: display.window.scale_factor() as f32,
        };
        self.rpass.update_texture(&display.device, &display.queue, &self.platform.context().texture());
        self.rpass.update_user_textures(&display.device, &display.queue);
        self.rpass.update_buffers(&display.device, &display.queue, &paint_jobs, &screen_descriptor);

        // Record all render passes.
        self.rpass.execute(
            &mut encoder,
            &output_frame.output.view,
            &paint_jobs,
            &screen_descriptor,
            None,
        );

        // Submit the commands.
        display.queue.submit(iter::once(encoder.finish()));
    }
}