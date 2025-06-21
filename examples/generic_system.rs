// #![cfg_attr(debug_assertions, allow(unused))]

use bevy::prelude::*;

fn main() {
    println!("\n#### generic_system ####\n");
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_systems(Startup, setup)
        .add_systems(Update, (change_app_state, log_on_enter))
        .add_systems(OnExit(AppState::Menu), cleanup_system::<MenuStateTag>)
        .add_systems(OnExit(AppState::InGame), cleanup_system::<InGameStateTag>)
        .run();
}

#[derive(Component)]
struct MenuStateTag;

#[derive(Component)]
struct InGameStateTag;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, States)]
enum AppState {
    #[default]
    Menu,
    InGame,
}

fn setup(mut cmd: Commands) {
    cmd.spawn(MenuStateTag);
    cmd.spawn(InGameStateTag);
}

fn change_app_state(mut app_state: ResMut<NextState<AppState>>, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::KeyQ) {
        app_state.set(AppState::Menu);
    } else if input.just_pressed(KeyCode::KeyW) {
        app_state.set(AppState::InGame);
    }
}

fn log_on_enter(app_state: Res<State<AppState>>) {
    if app_state.is_changed() {
        info!("AppState: {:?}", app_state.get());
    }
}

fn cleanup_system<T: Component>(mut cmd: Commands, query: Query<Entity, With<T>>) {
    for entity in query.iter() {
        info!("despawn {:?}", entity);
        cmd.entity(entity).despawn();
    }
}
