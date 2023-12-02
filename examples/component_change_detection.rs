// #![cfg_attr(debug_assertions, allow(unused))]

use bevy::prelude::*;

fn main() {
    println!("\n#### component_change_detection ####\n");

    use bevy::app::ScheduleRunnerPlugin;
    use bevy::log::LogPlugin;
    use std::time::Duration;

    App::new()
        .add_plugins((
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f32(0.2))),
            LogPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (modify_my_component, change_detection, tracker_monitoring),
        )
        .run();
}

#[derive(Component, PartialEq)]
struct MyComponent(f32);

fn setup(mut cmd: Commands) {
    cmd.spawn(MyComponent(0.));
    cmd.spawn(Transform::IDENTITY);
}

fn modify_my_component(time: Res<Time>, mut query: Query<(Entity, &mut MyComponent)>) {
    use rand::Rng;
    if rand::thread_rng().gen_bool(0.2) {
        let value = time.elapsed_seconds().round();
        for (entity, mut my_component) in query.iter_mut() {
            let new_component = MyComponent(value);
            info!(
                "{:?}'s MyComponent changed from {} to {}",
                entity, my_component.0, value
            );
            // Change detection occurs on mutable dereference,
            // and does not consider whether or not a value is actually equal.
            // To avoid triggering change detection when nothing has actually changed,
            // you can use the `set_if_neq` method on any component or resource that implements PartialEq
            my_component.set_if_neq(new_component);
        }
    }
}

fn change_detection(query: Query<Entity, Changed<MyComponent>>) {
    for entity in query.iter() {
        info!("{:?}'s MyComponent changed", entity);
    }
}

fn tracker_monitoring(query: Query<(Entity, Ref<MyComponent>)>) {
    for (entity, my_component) in query.iter() {
        let is_added = my_component.is_added();
        let is_changed = my_component.is_changed();
        info!(
            "{:?}'s MyComponent added {:>6}, changed {:>6}",
            entity, is_added, is_changed
        );
    }
}
