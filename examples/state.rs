#![cfg_attr(debug_assertions, allow(unused))]

use bevy::{core::Zeroable, prelude::*};

fn main() {
    println!("\n#### state ####\n");

    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_systems(Startup, setup)
        .add_systems(OnEnter(AppState::Menu), menu_setup)
        .add_systems(OnExit(AppState::Menu), menu_cleanup)
        .add_systems(OnEnter(AppState::InGame), spawn_sprite)
        .add_systems(Update, menu.run_if(in_state(AppState::Menu)))
        .add_systems(
            Update,
            (sprite_move, change_color).run_if(in_state(AppState::InGame)),
        )
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
enum AppState {
    #[default]
    Menu,
    InGame,
}

#[derive(Resource)]
struct MenuItem(Entity);

fn setup(mut cmd: Commands) {
    cmd.spawn(Camera2dBundle::default());
}

fn menu_setup(mut cmd: Commands) {
    let entity = cmd
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.),
                        height: Val::Px(65.),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    background_color: BUTTON_COLOR_DEFAULT.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play",
                        TextStyle {
                            font_size: 35.,
                            ..Default::default()
                        },
                    ));
                });
        })
        .id();

    cmd.insert_resource(MenuItem(entity));
}

const BUTTON_COLOR_DEFAULT: Color = Color::BLACK;
const BUTTON_COLOR_HOVER: Color = Color::SEA_GREEN;
const BUTTON_COLOR_PRESSED: Color = Color::ORANGE;

#[allow(clippy::type_complexity)]
fn menu(
    mut app_state: ResMut<NextState<AppState>>,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut bg_color) in button_query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                *bg_color = BUTTON_COLOR_PRESSED.into();
                app_state.set(AppState::InGame);
            }
            Interaction::Hovered => *bg_color = BUTTON_COLOR_HOVER.into(),
            Interaction::None => *bg_color = BUTTON_COLOR_DEFAULT.into(),
        };
    }
}

fn menu_cleanup(mut cmd: Commands, menu_item: Option<Res<MenuItem>>) {
    if let Some(item) = menu_item {
        cmd.entity(item.0).despawn_recursive();
    }
}

fn spawn_sprite(mut cmd: Commands, asset_server: Res<AssetServer>) {
    cmd.spawn(SpriteBundle {
        texture: asset_server.load("./branding/icon.png"),
        ..Default::default()
    });
}

const SPEED: f32 = 300.;

fn sprite_move(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Sprite>>,
) {
    for mut transform in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        if input.pressed(KeyCode::Up) {
            direction = Vec3::Y;
        }
        if input.pressed(KeyCode::Down) {
            direction = Vec3::Y * -1.;
        }
        if input.pressed(KeyCode::Right) {
            direction = Vec3::X;
        }
        if input.pressed(KeyCode::Left) {
            direction = Vec3::X * -1.;
        }

        if direction != Vec3::ZERO {
            transform.translation += direction.normalize() * SPEED * time.delta_seconds();
        }
    }
}

fn change_color(time: Res<Time>, mut query: Query<&mut Sprite>) {
    for mut sprite in &mut query {
        sprite
            .color
            .set_b((time.elapsed_seconds() * 0.5).sin() + 2.0);
    }
}
