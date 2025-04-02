//! In this modules location general systems of game

// import crates
use bevy::prelude::*;
use autodefault::autodefault;
use bevy_rapier3d::prelude::*;

// import this crate
use crate::components::SphereOfTear;

/// Setup system for bevy
#[autodefault(except(CameraComponent))]
pub fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>,
) {
    // spawn a 3d camera
    // commands.spawn((
    //     CameraComponent {},
    //     Camera3d::default(),
    //     Projection::from(PerspectiveProjection { fov: 90.0_f32.to_radians() }),
    //     Transform::from_xyz(-4.0, 5.5, 12.0)
    //         .looking_at(Vec3::ZERO, Vec3::Y)
    // ));

    // create a test sphere
    commands.spawn((
        SphereOfTear(4.0),
        Transform::from_xyz(2.0, 2.0, 2.0),
        Mesh3d(meshes.add(Sphere::new(2.0))),
        MeshMaterial3d(materials.add(Color::BLACK))
    ));

    // main tests scene
    commands.spawn((
        SceneRoot(assets.load("gltf/character_controller_demo2.glb#Scene0")),
        RigidBody::Fixed, Transform::default(),
        AsyncSceneCollider::default()
    ));

    // spawn a light
    commands.spawn((
       PointLight {
           shadows_enabled: true
       },
       Transform::from_xyz(4.0, 8.0, 4.0)
    ));
}

