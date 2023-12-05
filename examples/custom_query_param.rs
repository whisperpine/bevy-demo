#![cfg_attr(debug_assertions, allow(unused))]

use bevy::{ecs::query::WorldQuery, prelude::*, ui::debug};

fn main() {
    println!("\n#### custom_query_param ####\n");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, print_system)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

fn setup(mut cmd: Commands) {
    cmd.spawn((ComponentA, ComponentB, ComponentC));
}

fn print_system(query: Query<CustomQuery, QueryFilter>) {
    for e in query.iter() {
        println!("{:?}", e.a);
    }
}

#[derive(Component, Debug)]
struct ComponentA;

#[derive(Component, Debug)]
struct ComponentB;

#[derive(Component, Debug)]
struct ComponentC;

#[derive(Component, Debug)]
struct ComponentD;

#[derive(Component, Debug)]
struct ComponentE;

#[derive(WorldQuery)]
#[world_query(mutable, derive(Debug))]
struct CustomQuery {
    a: &'static ComponentA,
}

#[derive(WorldQuery)]
#[world_query(derive(Debug))]
struct QueryFilter {
    _a: With<ComponentA>,
    _b: With<ComponentB>,
    _c_or_d: Or<(With<ComponentC>, With<ComponentD>)>,
}
