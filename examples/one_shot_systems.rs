#![cfg_attr(debug_assertions, allow(unused))]

use bevy::{
    app::ScheduleRunnerPlugin,
    ecs::system::{RunSystemOnce, SystemId},
    prelude::*,
};

fn main() {
    println!("\n#### one_shot_systems ####\n");

    App::new()
        .add_plugins(ScheduleRunnerPlugin::run_loop(
            std::time::Duration::from_secs_f32(0.5),
        ))
        .add_systems(Startup, (count_entities, setup).chain())
        .add_systems(Update, trigger_callback)
        .add_systems(PostUpdate, count_entities)
        .run();
}

fn count_entities(query: Query<()>) {
    let entity_count = query.iter().count();
    dbg!(entity_count);
}

#[derive(Component)]
struct Callback(SystemId);

#[derive(Component)]
#[component(storage = "SparseSet")]
struct Trigger;

fn setup(world: &mut World) {
    // Register system and store system_id in Callback component.
    let press_button_system = world.register_system(press_button);
    world.spawn((Callback(press_button_system), Trigger));

    let toggle_switch_system = world.register_system(toggle_switch);
    world.spawn(Callback(toggle_switch_system));

    // Run system directly by world.
    world.run_system_once(count_entities);
}

fn press_button() {
    println!("button pressed");
}

fn toggle_switch() {
    println!("switch toggled");
}

fn trigger_callback(mut cmd: Commands, query: Query<(Entity, &Callback), With<Trigger>>) {
    for (entity, Callback(system_id)) in query.iter() {
        cmd.run_system(*system_id);
        cmd.entity(entity).remove::<Trigger>();
    }
}
