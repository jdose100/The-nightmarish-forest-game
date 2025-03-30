//! In this modules location general systems of game

// import crates
use bevy::prelude::*;
use avian3d::prelude::*;
use autodefault::autodefault;

// import this crate
use crate::camera::CameraComponent;
use crate::components::SphereOfTear;

/// Setup system for bevy
#[autodefault(except(CameraComponent))]
pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>
) {
    // spawn a 3d camera
    commands.spawn((
        CameraComponent {},
        Camera3d::default(),
        Projection::from(PerspectiveProjection { fov: 90.0_f32.to_radians() }),
        // Transform::from_xyz(-4.0, 5.5, 12.0)
        //     .looking_at(Vec3::ZERO, Vec3::Y)
    ));

    // create a test sphere
    commands.spawn((
        SphereOfTear(4.0),
        Transform::from_xyz(2.0, 2.0, 2.0),
        Mesh3d(meshes.add(Sphere::new(2.0))),
        MeshMaterial3d(materials.add(Color::BLACK))
    ));

    // // spawn a floor
    // commands.spawn((
    //     Mesh3d(meshes.add(Cylinder::new(9.0, 0.1))),
    //     MeshMaterial3d(materials.add(Color::WHITE)),
    //     Transform::from_xyz(0.0, -1.0, 0.0),
    //     RigidBody::Static,
    //     Collider::cylinder(9.0, 0.1)
    // ));

    // // spawn a wall
    // commands.spawn((
    //     Mesh3d(meshes.add(Cuboid::new(5.0, 5.0, 1.0))),
    //     MeshMaterial3d(materials.add(Color::srgb(1.0, 0.1, 0.0))),
    //     Transform::from_xyz(-1.0, 1.5, -4.0),
    //     RigidBody::Static,
    //     Collider::cuboid(5.0, 5.0, 1.0)
    // ));

    // main tests scene
    commands.spawn((
        SceneRoot(assets.load("gltf/character_controller_demo2.glb#Scene0")),
        Transform::default(),
        ColliderConstructorHierarchy::new(ColliderConstructor::ConvexHullFromMesh),
        RigidBody::Static,
    ));

    // test platform for rapier
    // commands.spawn((
    //     Mesh3d(meshes.add(Cuboid::new(40.0, 0.7, 40.0))),
    //     MeshMaterial3d(materials.add(Color::srgb(0.5, 0.2, 0.9))),
    // ));

    // spawn a light
    commands.spawn((
       PointLight {
           shadows_enabled: true
       },
       Transform::from_xyz(4.0, 8.0, 4.0)
    ));
}

