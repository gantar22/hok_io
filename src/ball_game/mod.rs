use std::f32::consts::TAU;
use bevy::prelude::*;
use bevy::render::mesh::CircleMeshBuilder;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy_rapier2d::parry::shape::ShapeType::Capsule;
use bevy_rapier2d::prelude::*;

pub struct BallGamePlugin;

impl Plugin for BallGamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            //.add_plugins(RapierDebugRenderPlugin::default())
            .add_systems(Startup,setup_demo)
            .add_systems(Update,tick_demo)
        ;
    }
}

#[derive(Component)]
struct DemoPlayer;

fn setup_demo(
    mut commands: Commands,
    mut rapier_conf: ResMut<RapierConfiguration>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    rapier_conf.gravity = Vec2::ZERO;
    const STICK_LENGTH: f32 = 35.0;
    const STICK_RADIUS: f32 = 5.0;

    let stick = commands
        .spawn(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Capsule2d {
                radius: STICK_RADIUS,
                half_length: STICK_LENGTH,
            })),
            material: materials.add(Color::linear_rgb(1.0,0.0,0.0)),
            transform: Transform::from_rotation(Quat::from_euler(EulerRot::YXZ, 0.0, 0.0,TAU * 0.25)).with_translation(Vec3::new(20.0, 0.0, 0.0)),
            ..Default::default()
        })
        .insert(Collider::capsule(Vect::new(0.0,0.0),Vect::new(0.0,STICK_LENGTH),STICK_RADIUS))
        .id();

    const PLAYER_RADIUS: f32 = 20.0;

    let player = commands
        .spawn(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle { radius: PLAYER_RADIUS })),
            material: materials.add(Color::linear_rgb(1.0, 0.0, 0.0)),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(PLAYER_RADIUS))
        .insert(Restitution::coefficient(1.0))
        .add_child(stick)
        .insert(DemoPlayer)
        .insert(ExternalForce {
            force: Vec2::new(0.0,0.0),
            torque: 0.0,
        })
        .id();

    const BALL_RADIUS: f32 = 7.0;

    let ball = commands
        .spawn(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle { radius: BALL_RADIUS })),
            material: materials.add(Color::linear_rgb(1.0, 1.0, 1.0)),
            transform: Transform::from_xyz(100.0, 0.0, 0.0),
            ..default()
        });
}

fn tick_demo(
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&DemoPlayer,&mut ExternalForce)>,
) {
    for (_, mut forces) in player_query.iter_mut() {
        if keys.pressed(KeyCode::KeyA) {
            forces.force += Vec2::new(10.0, 0.0);
        }
    }
}