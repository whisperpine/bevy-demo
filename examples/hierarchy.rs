// #![cfg_attr(debug_assertions, allow(unused))]

use bevy::prelude::*;

fn main() {
    println!("\n#### hierarchy ####\n");

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn)
        .add_systems(Update, (rotate, despawn_later))
        .run();
}

fn spawn(mut cmd: Commands, asset_server: Res<AssetServer>) {
    use bevy::color::palettes::css::ORANGE;
    cmd.spawn(Camera2d);
    let texture = asset_server.load("branding/icon.png");
    cmd.spawn((
        Sprite::from_image(texture.clone()),
        children![
            (
                Sprite::from_image(texture.clone()),
                Transform::from_xyz(300., 0., 0.).with_scale(Vec3::splat(0.5)),
            ),
            (
                Sprite {
                    image: texture.clone(),
                    color: ORANGE.into(),
                    ..default()
                },
                Transform::from_xyz(0., 300., 0.).with_scale(Vec3::splat(0.5)),
            )
        ],
    ));
}

fn rotate(
    time: Res<Time>,
    parent_query: Query<(Entity, &Children), With<Sprite>>,
    mut transform_query: Query<&mut Transform, With<Sprite>>,
) {
    use std::f32::consts::TAU;

    for (entity, children) in parent_query.iter() {
        if let Ok(mut transform) = transform_query.get_mut(entity) {
            transform.rotate_z(TAU * 0.1 * time.delta_secs());
        }

        for child in children {
            if let Ok(mut transform) = transform_query.get_mut(*child) {
                transform.rotate_z(TAU * 0.5 * time.delta_secs());
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
    query: Query<Entity, (With<Sprite>, With<ChildOf>)>,
) {
    if !despawn_timer.0.tick(time.delta()).just_finished() {
        return;
    }
    if let Some(entity) = query.iter().next() {
        println!("elapsed seconds: {:.3}", time.elapsed_secs());
        cmd.entity(entity).despawn();
    }
}
