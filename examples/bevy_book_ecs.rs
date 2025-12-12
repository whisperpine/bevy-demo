// #![cfg_attr(debug_assertions, allow(unused))]

use bevy::prelude::*;

fn main() {
    println!("\n#### bevy_book_ecs ####\n");
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

fn greet_people(
    time: Res<Time>,
    mut greet_timer: ResMut<GreetTimer>,
    query: Query<&Player, With<Person>>,
) {
    if greet_timer.0.tick(time.delta()).just_finished() {
        for player in &query {
            println!("hello {}!", player.name);
        }
        println!();
    }
}

#[derive(Component, Default)]
struct Person;

#[derive(Component)]
#[require(Person)]
struct Player {
    name: String,
}

impl Player {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
        }
    }
}

/// Press `Ecs` to close focussed window.
fn close_on_esc(
    mut cmd: Commands,
    query: Query<(Entity, &Window)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (entity, window) in query.iter() {
        if !window.focused {
            continue;
        }
        if input.just_pressed(KeyCode::Escape) {
            cmd.entity(entity).despawn();
        }
    }
}
