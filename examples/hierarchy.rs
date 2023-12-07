#![cfg_attr(debug_assertions, allow(unused))]

use bevy::{core::FrameCount, prelude::*};

fn main() {
    println!("\n#### hierarchy ####\n");

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn)
        .add_systems(Update, (bevy::window::close_on_esc, rotate, despawn_later))
        .run();
}

fn spawn(mut cmd: Commands, asset_server: Res<AssetServer>) {
    cmd.spawn(Camera2dBundle::default());

    let texture = asset_server.load("branding/icon.png");
    let parent = cmd
        .spawn(SpriteBundle {
            texture: texture.clone(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(SpriteBundle {
                texture: texture.clone(),
                transform: Transform::from_xyz(300., 0., 0.).with_scale(Vec3::splat(0.5)),
                sprite: Sprite {
                    color: Color::SEA_GREEN,
                    ..default()
                },
                ..default()
            });
        })
        .id();

    let another_child = cmd
        .spawn(SpriteBundle {
            texture: texture.clone(),
            transform: Transform::from_xyz(0., 300., 0.).with_scale(Vec3::splat(0.5)),
            sprite: Sprite {
                color: Color::ORANGE,
                ..default()
            },
            ..default()
        })
        .id();

    cmd.entity(parent).add_child(another_child);
}

fn rotate(
    time: Res<Time>,
    parent_query: Query<(Entity, &Children), With<Sprite>>,
    mut transform_query: Query<&mut Transform, With<Sprite>>,
) {
    use std::f32::consts::TAU;

    for (entity, children) in parent_query.iter() {
        if let Ok(mut transform) = transform_query.get_mut(entity) {
            transform.rotate_z(TAU * 0.1 * time.delta_seconds());
        }

        for child in children {
            if let Ok(mut transform) = transform_query.get_mut(*child) {
                transform.rotate_z(TAU * 0.5 * time.delta_seconds());
            }
        }
    }
}

#[derive(Resource)]
struct DespawnTimer(Timer);

impl Default for DespawnTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(1.5, TimerMode::Repeating))
    }
}

fn despawn_later(
    mut cmd: Commands,
    mut despawn_timer: Local<DespawnTimer>,
    time: Res<Time>,
    query: Query<Entity, (With<Sprite>, With<Parent>)>,
) {
    if despawn_timer.0.tick(time.delta()).just_finished() {
        if let Some(entity) = query.iter().next() {
            println!("elapsed seconds: {:.3}", time.elapsed_seconds());
            cmd.entity(entity).despawn();
        }
    }
}
