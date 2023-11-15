// #![cfg_attr(debug_assertions, allow(unused))]

use bevy::prelude::*;

fn main() {
    App::new().add_plugins((DefaultPlugins, HelloPlugin)).run();
}
struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Startup, add_people)
            .add_systems(Update, greet_people);
    }
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("amiao".to_owned())));
    commands.spawn((Person, Name("yusong".to_owned())));
}

#[derive(Resource)]
struct GreetTimer(Timer);

fn greet_people(res: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(res.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", **name);
        }
    }
}

#[derive(Component)]
struct Person;

#[derive(Component, Deref)]
struct Name(String);
