use std::{iter, time::Instant};

use egui::FontDefinitions;
use egui_wgpu_backend::{RenderPass, ScreenDescriptor};
use egui_winit_platform::{Platform, PlatformDescriptor};
use wgpu::TextureView;

use crate::{
    rendering::{
        gui_renderer::{GridApp, PerfApp, StatApp},
        Display, PetriEventHandler,
    },
    simulation::Simulation,
};

pub struct GUIRenderer {
    platform: Platform,
    rpass: RenderPass,
    start_time: Instant,
    previous_frame_time: Option<f32>,
    debug: StatApp,
    grid: GridApp,
    perf: PerfApp,
}

impl GUIRenderer {
    pub fn new(display: &Display, _simulation: &mut Simulation) -> Self {
        let size = display.window.inner_size();
        // We use the egui_winit_platform crate as the platform.
        let platform = Platform::new(PlatformDescriptor {
            physical_width: size.width as u32,
            physical_height: size.height as u32,
            scale_factor: display.window.scale_factor(),
            font_definitions: FontDefinitions::default(),
            style: Default::default(),
        });

        // We use the egui_wgpu_backend crate as the render backend.
        let egui_rpass = RenderPass::new(&display.device, display.surface_config.format, 1);

        Self {
            platform,
            rpass: egui_rpass,
            start_time: Instant::now(),
            previous_frame_time: None,
            debug: StatApp,
            grid: GridApp::default(),
            perf: PerfApp,
        }
    }

    pub fn render(&mut self, display: &Display, simulation: &Simulation, view: &TextureView) {
        self.platform.update_time(self.start_time.elapsed().as_secs_f64());

        // Begin to draw the UI frame.
        let egui_start = Instant::now();
        self.platform.begin_frame();

        self.grid.update(&self.platform.context(), display, simulation);
        self.debug.update(&self.platform.context(), display, simulation);
        self.perf.update(&self.platform.context(), display, simulation);

        // End the UI frame. We could now handle the output and draw the UI with the backend.
        let (_output, paint_commands) = self.platform.end_frame(Some(&display.window));
        let paint_jobs = self.platform.context().tessellate(paint_commands);

        let frame_time = (Instant::now() - egui_start).as_secs_f64() as f32;
        self.previous_frame_time = Some(frame_time);

        let mut encoder = display
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("encoder") });

        // Upload all resources for the GPU.
        let screen_descriptor = ScreenDescriptor {
            physical_width: display.surface_config.width,
            physical_height: display.surface_config.height,
            scale_factor: display.window.scale_factor() as f32,
        };
        self.rpass
            .update_texture(&display.device, &display.queue, &self.platform.context().texture());
        self.rpass.update_user_textures(&display.device, &display.queue);
        self.rpass
            .update_buffers(&display.device, &display.queue, &paint_jobs, &screen_descriptor);

        // Record all render passes.
        self.rpass
            .execute(&mut encoder, view, &paint_jobs, &screen_descriptor, None)
            .unwrap();

        // Submit the commands.
        display.queue.submit(iter::once(encoder.finish()));
    }
}

impl PetriEventHandler for GUIRenderer {
    fn forward_event<T>(
        &mut self,
        _display: &mut Display,
        _simulation: &mut Simulation,
        event: &winit::event::Event<T>,
    ) {
        self.platform.handle_event(event)
    }
}
