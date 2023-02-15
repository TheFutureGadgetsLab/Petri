use ultraviolet::Vec2;

#[derive(Default)]
pub struct Camera {
    pub world_xbounds: Vec2,
    pub world_ybounds: Vec2,
    pub screen_height: f32,
    pub screen_width: f32,
}

impl Camera {
    pub fn update(&mut self, plot_ui: &egui::plot::PlotUi) {
        let bounds = plot_ui.plot_bounds();
        let (minx, miny) = (bounds.min()[0], bounds.min()[1]);
        let (maxx, maxy) = (bounds.max()[0], bounds.max()[1]);
        self.world_xbounds = Vec2::new(minx as _, maxx as _);
        self.world_ybounds = Vec2::new(miny as _, maxy as _);
        self.screen_height = (plot_ui.screen_from_plot([minx, miny].into())
            - plot_ui.screen_from_plot([minx, maxy].into()))
        .round()
        .y;
        self.screen_width = (plot_ui.screen_from_plot([maxx, miny].into())
            - plot_ui.screen_from_plot([minx, miny].into()))
        .round()
        .x;
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, Default)]
pub struct CameraUniform {
    u_world_size: [f32; 2],
    u_world_ll: [f32; 2],
    u_screen_height: f32,
    u_screen_width: f32,
}

impl From<&Camera> for CameraUniform {
    fn from(cam: &Camera) -> Self {
        CameraUniform {
            u_world_size: [
                cam.world_xbounds.y - cam.world_xbounds.x,
                cam.world_ybounds.y - cam.world_ybounds.x,
            ],
            u_world_ll: [cam.world_xbounds.x, cam.world_ybounds.x],
            u_screen_height: cam.screen_height,
            u_screen_width: cam.screen_width,
        }
    }
}
