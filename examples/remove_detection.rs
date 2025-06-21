// #![cfg_attr(debug_assertions, allow(unused))]

use bevy::log::LogPlugin;
use bevy::prelude::*;

const CRATE_NAME: &str = env!("CARGO_CRATE_NAME");

fn main() {
    println!("\n#### remove_detection ####\n");

    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            filter: format!("wgpu=error,naga=warn,{CRATE_NAME}=debug"),
            ..Default::default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, (remove_later, react_on_removal))
        .run();
}

#[derive(Component)]
struct MyComponent;

fn setup(mut cmd: Commands, asset_server: Res<AssetServer>) {
    cmd.spawn(Camera2d);
    cmd.spawn((
        Sprite::from_image(asset_server.load("branding/icon.png")),
        MyComponent,
    ));
}

fn remove_later(
    mut cmd: Commands,
    time: Res<Time>,
    query: Query<Entity, (With<Sprite>, With<MyComponent>)>,
) {
    if time.elapsed_secs() > 1.5 {
        for entity in query.iter() {
            cmd.entity(entity).remove::<MyComponent>();
            debug!("{:?}'s MyComponent has been removed", entity);
        }
    }
}

fn react_on_removal(mut removal: RemovedComponents<MyComponent>, mut query: Query<&mut Sprite>) {
    for entity in removal.read() {
        if let Ok(mut sprite) = query.get_mut(entity) {
            use bevy::color::palettes::css::SEA_GREEN;
            sprite.color = Color::Srgba(SEA_GREEN);
        }
    }
}
