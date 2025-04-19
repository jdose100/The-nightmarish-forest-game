//! In this modules location general systems of game

// import crates
use bevy::prelude::*;
use autodefault::autodefault;
use bevy_rapier3d::prelude::*;

// import this crate
use crate::components::{CanPickable, InInventory, SphereOfTear};

/// Setup system for bevy
#[autodefault]
pub fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut picking_settings: ResMut<MeshPickingSettings>,
    assets: Res<AssetServer>,
) {
    // spawn a 3d camera
    // commands.spawn((
    //     Camera3d::default(),
    //     Projection::from(PerspectiveProjection { fov: 90.0_f32.to_radians() }),
    //     Transform::from_xyz(-4.0, 5.5, 12.0)
    //         .looking_at(Vec3::ZERO, Vec3::Y)
    // ));

    // setup picking settings
    picking_settings.require_markers = false;

    // add pickable cube
    commands.spawn((
        Name::new("some cube"),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.1, 0.3))),
        Mesh3d(meshes.add(Cuboid::new(0.7, 0.7, 0.7))),
        Transform::from_xyz(4.0, 4.0, 2.0),
        CanPickable
    )).observe(picking);

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
        // SceneRoot(assets.load("gltf/spawn.glb#Scene0")),
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

/// interactive systems for picking objects
fn picking(
    _drag: Trigger<Pointer<Click>>,
    query: Query<(Entity, &mut Transform), With<CanPickable>>,
    mut commands: Commands,
) {
    for entity in query.iter() {
        commands.entity(entity.0).remove::<CanPickable>();
        commands.entity(entity.0).insert(InInventory);
    }
}

pub fn in_inventory(query: Query<&InInventory>) {
    for _ in query.iter() {
        info!("in inventoty some!");
    }
}

