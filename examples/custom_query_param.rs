// #![cfg_attr(debug_assertions, allow(unused))]

use std::fmt::Debug;

use bevy::ecs::query::{QueryData, QueryFilter};
use bevy::prelude::*;

fn main() {
    println!("\n#### custom_query_param ####\n");

    use bevy::app::ScheduleRunnerPlugin;
    App::new()
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(
            std::time::Duration::from_secs_f32(0.5),
        )))
        .add_systems(Startup, setup)
        .add_systems(Update, print_system)
        .run();
}

fn setup(mut cmd: Commands) {
    cmd.spawn((
        ComponentA {
            name: "amiao".to_owned(),
        },
        ComponentB(0),
        ComponentC,
        ComponentD,
    ));
    cmd.spawn((
        ComponentA {
            name: "yahaha".to_owned(),
        },
        ComponentC,
        ComponentD,
    ));
}

fn print_system(mut query: Query<CustomQuery<ComponentC, ComponentD>, MyQueryFilter>) {
    for e in query.iter_mut() {
        println!("{:?}", e.entity);
        println!("{:?}", e.a.name);
        if let Some(mut component_b) = e.b {
            **component_b += 1;
            println!("{:?}", component_b);
        }
        println!("{:?}", e.generic.value);
    }
    println!();
}

#[derive(Component, Debug)]
struct ComponentA {
    name: String,
}

#[derive(Component, Debug, Deref, DerefMut)]
struct ComponentB(u32);

#[derive(Component, Debug)]
struct ComponentC;

#[derive(Component, Debug)]
struct ComponentD;

#[derive(QueryData)]
#[query_data(mutable, derive(Debug))]
struct CustomQuery<T, U>
where
    T: Component + Debug,
    U: Component + Debug,
{
    entity: Entity,
    a: &'static ComponentA,
    b: Option<&'static mut ComponentB>,
    generic: GenericQuery<T, U>,
}

#[derive(QueryData)]
#[query_data(derive(Debug))]
struct GenericQuery<T: Component, U: Component> {
    value: (&'static T, &'static U),
}

#[derive(QueryFilter)]
struct MyQueryFilter {
    _a: With<ComponentA>,
    _c_or_d: Or<(With<ComponentC>, With<ComponentD>)>,
}
