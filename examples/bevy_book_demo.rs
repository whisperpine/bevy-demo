// #![cfg_attr(debug_assertions, allow(unused))]

use bevy::prelude::*;

fn main() {
    println!("\n#### bevy_book_demo ####\n");
    App::new()
        .add_plugins((DefaultPlugins, HelloPlugin))
        .add_systems(Update, close_on_esc)
        .run();
}

struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Startup, (print_hints, add_people))
            .add_systems(Update, greet_people);
    }
}

fn print_hints() {
    println!("Press Ctrl+C to exit.\n");
}

fn add_people(mut commands: Commands) {
    commands.spawn_batch([
        Player::new("amiao"),
        Player::new("yusong"),
        Player::new("yahaha"),
    ]);
}

#[derive(Resource)]
struct GreetTimer(Timer);

fn greet_people(res: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(res.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", **name);
        }
        println!();
    }
}

#[derive(Component)]
struct Person;

#[derive(Component, Deref)]
struct Name(String);

#[derive(Bundle)]
struct Player {
    person: Person,
    name: Name,
}

impl Player {
    fn new(name: &str) -> Self {
        Self {
            person: Person,
            name: Name(name.to_owned()),
        }
    }
}

/// Press `Ecs` to close focussed window.
///
/// The same as [`bevy::window::close_on_esc`].
fn close_on_esc(mut cmd: Commands, query: Query<(Entity, &Window)>, input: Res<Input<KeyCode>>) {
    for (entity, window) in query.iter() {
        if !window.focused {
            continue;
        } else if input.just_pressed(KeyCode::Escape) {
            cmd.entity(entity).despawn();
        }
    }
}
