mod mesh;
use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
    sprite::Mesh2dHandle,
};
use mesh::{ColoredMesh2d, ColoredMesh2dPlugin};

/// This example shows how to manually render 2d items using "mid level render apis" with a custom pipeline for 2d meshes
/// It doesn't use the [`Material2d`] abstraction, but changes the vertex buffer to include vertex color
/// Check out the "mesh2d" example for simpler / higher level 2d meshes
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ColoredMesh2dPlugin)
        .add_startup_system(star)
        .run();
}

fn star(
    mut commands: Commands,
    // We will add a new Mesh for the star being created
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // Let's define the mesh for the object we want to draw: a nice star.
    // We will specify here what kind of topology is used to define the mesh,
    // that is, how triangles are built from the vertices. We will use a
    // triangle list, meaning that each vertex of the triangle has to be
    // specified.
    let mut star = Mesh::new(PrimitiveTopology::TriangleList);

    // Vertices need to have a position attribute. We will use the following
    // vertices (I hope you can spot the star in the schema).
    //
    //        1
    //
    //     10   2
    // 9      0      3
    //     8     4
    //        6
    //   7        5
    //
    // These vertices are specificed in 3D space.
    let mut v_pos = vec![[0.0, 0.0, 0.0]];
    for i in 0..10 {
        // Angle of each vertex is 1/10 of TAU, plus PI/2 for positioning vertex 0
        let a = std::f32::consts::FRAC_PI_2 - i as f32 * std::f32::consts::TAU / 10.0;
        // Radius of internal vertices (2, 4, 6, 8, 10) is 100, it's 200 for external
        let r = (1 - i % 2) as f32 * 100.0 + 100.0;
        // Add the vertex coordinates
        v_pos.push([r * a.cos(), r * a.sin(), 0.0]);
    }
    // Set the position attribute
    star.set_attribute(Mesh::ATTRIBUTE_POSITION, v_pos);
    // And a RGB color attribute as well
    let mut v_color = vec![[0.0, 0.0, 0.0, 1.0]];
    v_color.extend_from_slice(&[[1.0, 1.0, 0.0, 1.0]; 10]);
    star.set_attribute(Mesh::ATTRIBUTE_COLOR, v_color);

    // Now, we specify the indices of the vertex that are going to compose the
    // triangles in our star. Vertices in triangles have to be specified in CCW
    // winding (that will be the front face, colored). Since we are using
    // triangle list, we will specify each triangle as 3 vertices
    //   First triangle: 0, 2, 1
    //   Second triangle: 0, 3, 2
    //   Third triangle: 0, 4, 3
    //   etc
    //   Last triangle: 0, 1, 10
    let mut indices = vec![0, 1, 10];
    for i in 2..=10 {
        indices.extend_from_slice(&[0, i, i - 1]);
    }
    star.set_indices(Some(Indices::U32(indices)));

    // We can now spawn the entities for the star and the camera
    commands.spawn_bundle((
        // We use a marker component to identify the custom colored meshes
        ColoredMesh2d::default(),
        // The `Handle<Mesh>` needs to be wrapped in a `Mesh2dHandle` to use 2d rendering instead of 3d
        Mesh2dHandle(meshes.add(star)),
        // These other components are needed for 2d meshes to be rendered
        Transform::default(),
        GlobalTransform::default(),
        Visibility::default(),
        ComputedVisibility::default(),
    ));
    commands
        // And use an orthographic projection
        .spawn_bundle(OrthographicCameraBundle::new_2d());
}
