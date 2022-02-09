mod components;
mod rendering;

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, math::Quat, prelude::*, render::camera::Camera};
use components::{CellBundle, ColorComp};
use rand::Rng;

use crate::rendering::{CellRenderPlugin, GuiRenderPlugins};

const CAMERA_SPEED: f32 = 1000.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_plugins(GuiRenderPlugins)
        .add_plugin(CellRenderPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_startup_system(setup)
        .add_system(move_camera)
        .run();
}

fn setup(mut commands: Commands) {
    let mut rng = rand::thread_rng();

    let tile_size = Vec2::splat(64.0);
    let map_size = Vec2::splat(320.0);

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
