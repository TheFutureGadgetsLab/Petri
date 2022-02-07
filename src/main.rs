mod cell_render;
mod components;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    math::Quat,
    prelude::*,
    render::camera::Camera,
};
use components::{CellBundle, ColorComp};
use rand::Rng;

const CAMERA_SPEED: f32 = 1000.0;

fn main() {
    App::new()
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(cell_render::CellRenderPlugin)
        .add_startup_system(setup)
        .add_system(move_camera)
        .run()
}

fn setup(mut commands: Commands) {
    let mut rng = rand::thread_rng();

    let tile_size = Vec2::splat(64.0);
    let map_size = Vec2::splat(320.0 * 4.0);

    let half_x = (map_size.x / 2.0) as i32;
    let half_y = (map_size.y / 2.0) as i32;

    // Spawns the camera
    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d())
        .insert(Transform::from_xyz(0.0, 0.0, 1000.0));

    let mut cells = vec![];
    let mut count = 0;
    for y in -half_y..half_y {
        for x in -half_x..half_x {
            count += 1;
            let position = Vec2::new(x as f32, y as f32);
            let translation = (position * tile_size).extend(rng.gen::<f32>());
            let rotation = Quat::from_rotation_z(rng.gen::<f32>());
            let scale = Vec3::splat(rng.gen::<f32>() * 40.0);

            cells.push(CellBundle {
                transform: Transform {
                    translation,
                    rotation,
                    scale,
                },
                color: ColorComp::new_rand(&mut rng),
                ..Default::default()
            });
        }
    }
    commands.spawn_batch(cells);
    info!("Spawned {} cells", count);
}

// System for rotating and translating the camera
fn move_camera(time: Res<Time>, mut camera_query: Query<&mut Transform, With<Camera>>) {
    let mut camera_transform = camera_query.single_mut();
    camera_transform.rotate(Quat::from_rotation_z(time.delta_seconds() * 0.5));
    *camera_transform = *camera_transform * Transform::from_translation(Vec3::X * CAMERA_SPEED * time.delta_seconds());
}

struct PrintingTimer(Timer);

impl Default for PrintingTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(1.0, true))
    }
}
