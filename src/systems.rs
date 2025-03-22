// import crates
use bevy::prelude::*;
use avian3d::{math::*, prelude::*};
use autodefault::autodefault;

// import this crate
use crate::player::PlayerComponent;
use crate::player::character_controller::CharacterControllerBundle;

#[derive(Component)]
pub struct MeshComponent;

/// Setup system for bevy
#[autodefault]
pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {    
    // spawn 3d camera
    commands.spawn((
        Camera3d::default(),
        Projection::from(PerspectiveProjection { fov: 90.0_f32.to_radians() }),
        Transform::from_xyz(-4.0, 5.5, 12.0)
            .looking_at(Vec3::ZERO, Vec3::Y)
    ));

    // spawn mesh
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(9.0, 0.1))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, -1.0, 0.0),
        RigidBody::Static,
        Collider::cylinder(9.0, 0.1)
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(5.0, 1.0, 5.0))),
        MeshMaterial3d(materials.add(Color::srgb(1.0, 0.1, 0.0))),
        Transform {
            translation: Vec3::new(-1.0, 1.5, -4.0),
            rotation: Quat::from_euler(EulerRot::XYZ, 90.0_f32.to_radians(), 1.0_f32.to_radians(), 1.0_f32.to_radians())
        },
        RigidBody::Static,
        Collider::cuboid(5.0, 1.0, 5.0)
    ));

    // commands.spawn((
    //     MeshComponent,
    //     Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
    //     MeshMaterial3d(materials.add(Color::srgb(0.5, 0.2, 1.0))),
    //     Transform::default()
    // ));

    // spawn character controller
    commands.spawn((
        PlayerComponent,
        Mesh3d(meshes.add(Capsule3d::new(0.4, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
        Transform::default(),
        CharacterControllerBundle::
            new(Collider::capsule(0.39, 1.0), Vector::NEG_Y * 9.81 * 2.0)
            .with_movement(30.0, 0.92, 7.7, (30.0 as Scalar).to_radians())
    ));

    // spawn light
    commands.spawn((
       PointLight {
           shadows_enabled: true
       },
       Transform::from_xyz(4.0, 8.0, 4.0)
    ));
}

/// camera update system
pub fn camera_update(
    mut camera_transform: Single<&mut Transform, (With<Camera3d>, Without<PlayerComponent>)>,
    player_transform: Single<&Transform, (With<PlayerComponent>, Without<Camera3d>)>,
) {
    // move camera to player position and rotate camera with player rotation
    camera_transform.translation = player_transform.translation + Vec3::new(0.0, 1.0, 0.01);
    // camera_transform.rotation = player_transform.rotation;
}

