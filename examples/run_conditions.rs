//! This example demonstrates how to use run conditions to control when systems run.

// #![cfg_attr(debug_assertions, allow(unused))]

use std::fmt::Debug;

use bevy::prelude::*;

fn main() {
    println!("\n#### run_conditions ####\n");

    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<InputCounter>()
        .add_systems(Update, log_input)
        .add_systems(
            Update,
            count_input
                .run_if(resource_exists::<InputCounter>)
                .run_if(resource_exists::<Unused>.or(has_user_input)),
        )
        .add_systems(
            Update,
            print_input_counter
                .after(count_input)
                .run_if(
                    resource_exists::<InputCounter>.and(|input_counter: Res<InputCounter>| {
                        input_counter.is_changed() && !input_counter.is_added()
                    }),
                ),
        )
        .add_systems(
            Update,
            print_time_message
                .run_if(is_time_passed(2.))
                .run_if(not(is_time_passed(2.1))),
        )
        .run();
}

#[derive(Resource, Debug, Default, Deref, DerefMut)]
struct InputCounter(u32);

#[derive(Resource)]
struct Unused;

fn count_input(mut input_counter: ResMut<InputCounter>) {
    **input_counter += 1;
}

fn print_input_counter(input_counter: ResMut<InputCounter>) {
    println!("{:?}", *input_counter);
}

fn log_input(keyboard: Res<ButtonInput<KeyCode>>, mouse: Res<ButtonInput<MouseButton>>) {
    for key in keyboard.get_just_pressed() {
        println!("KeyCode: {key:?}");
    }
    for mouse in mouse.get_just_pressed() {
        println!("MouseButton: {mouse:?}");
    }
}

fn has_user_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
) -> bool {
    keyboard.just_pressed(KeyCode::Space)
        || mouse.any_just_pressed([MouseButton::Left, MouseButton::Right])
}

fn print_time_message() {
    println!("It has been more than 2 seconds since the program started and less than 2.1 seconds");
}

fn is_time_passed(cmp_time: f32) -> impl FnMut(Res<Time>) -> bool {
    move |time: Res<Time>| time.elapsed_secs() > cmp_time
}
