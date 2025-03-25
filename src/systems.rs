// import crates
use bevy::prelude::*;
use avian3d::prelude::*;
use autodefault::autodefault;

// import this crate
// use crate::player::character_controller::CharacterControllerBundle;
use crate::camera::CameraComponent;

/// Setup system for bevy
#[autodefault(except(CameraComponent))]
pub(crate) fn setup(
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>
) {    
    // spawn a 3d camera
    commands.spawn((
        CameraComponent {},
        Camera3d::default(),
        Projection::from(PerspectiveProjection { fov: 90.0_f32.to_radians() }),
        Transform::from_xyz(-4.0, 5.5, 12.0)
            .looking_at(Vec3::ZERO, Vec3::Y)
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

    commands.spawn((
        SceneRoot(assets.load("gltf/character_controller_demo.glb#Scene0")),
        Transform::default(),
        ColliderConstructorHierarchy::new(ColliderConstructor::ConvexHullFromMesh),
        RigidBody::Static
    ));

    // spawn character controller
    // commands.spawn((
    //     PlayerComponent,
    //     Mesh3d(meshes.add(Capsule3d::new(0.4, 1.0))),
    //     MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
    //     Transform::from_xyz(4.0, 10.0, 4.0),
    // ));

    // spawn a light
    commands.spawn((
       PointLight {
           shadows_enabled: true
       },
       Transform::from_xyz(4.0, 8.0, 4.0)
    ));
}

