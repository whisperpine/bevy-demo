// #![cfg_attr(debug_assertions, allow(unused))]

use bevy::prelude::*;

fn main() {
    println!("\n#### parallel_query ####\n");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_system)
        .add_systems(Update, (move_system, bounce_system))
        .run();
}

#[derive(Component)]
struct Velocity(Vec2);

fn spawn_system(mut cmd: Commands, asset_server: Res<AssetServer>) {
    use rand::Rng;
    let mut rng = rand::rng();

    const SPEED: f32 = 500.;
    let texture = asset_server.load("branding/icon.png");

    cmd.spawn(Camera2d);
    for _ in 0..1024 {
        let direction =
            bevy::math::vec2(rng.random_range(-1.0..1.0), rng.random_range(-1.0..1.0)).normalize();
        cmd.spawn((
            Sprite::from_image(texture.clone()),
            Velocity(direction * SPEED),
        ));
    }
}

fn move_system(time: Res<Time>, mut sprites: Query<(&mut Transform, &Velocity), With<Sprite>>) {
    sprites
        .par_iter_mut()
        .for_each(|(mut transform, velocity)| {
            transform.translation += velocity.0.extend(0.) * time.delta_secs();
        });
}

fn bounce_system(
    window: Query<&Window>,
    mut sprites: Query<(&Transform, &mut Velocity), With<Sprite>>,
) -> Result {
    let window = window.single()?;
    let width = window.width();
    let height = window.height();
    let left = width / -2.;
    let right = width / 2.;
    let bottom = height / -2.;
    let top = height / 2.;

    use bevy::ecs::batching::BatchingStrategy;
    sprites
        .par_iter_mut()
        .batching_strategy(BatchingStrategy::fixed(64))
        .for_each(|(transform, mut velocity)| {
            let translation = transform.translation;
            if !(left < translation.x
                && translation.x < right
                && bottom < translation.y
                && translation.y < top)
            {
                velocity.0 *= -1.;
            }
        });
    Ok(())
}
