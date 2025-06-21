// #![cfg_attr(debug_assertions, allow(unused))]

use bevy::prelude::*;

fn main() {
    println!("\n#### fixed_timestep ####\n");
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Time::<Fixed>::from_seconds(0.5))
        .add_systems(Update, log_update)
        .add_systems(FixedUpdate, log_fixed_update)
        .run();
}

fn log_update(mut previous: Local<f32>, time: Res<Time>) {
    info!("log_update: {}", time.elapsed_secs() - *previous);
    *previous = time.elapsed_secs();
}

fn log_fixed_update(mut previous: Local<f32>, time: Res<Time>, fixed_time: Res<Time<Fixed>>) {
    info!("log_fixed_update: {}", time.elapsed_secs() - *previous);
    let overstep = fixed_time.overstep().as_secs_f32();
    info!("overstep: {}", overstep);
    *previous = time.elapsed_secs();
}
