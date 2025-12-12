// #![cfg_attr(debug_assertions, allow(unused))]

use bevy::{log::LogPlugin, prelude::*};

fn main() {
    println!("\n#### level_up_ecs ####\n");
    const CRATE_NAME: &str = env!("CARGO_CRATE_NAME");

    App::new()
        .add_plugins((
            MyGamePlugin,
            MinimalPlugins,
            LogPlugin {
                filter: format!("wgpu=error,naga=warn,{CRATE_NAME}=debug"),
                ..Default::default()
            },
        ))
        .run();
}

struct MyGamePlugin;

impl Plugin for MyGamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AddXpTimer>()
            .insert_resource(GameRules { level_up_xp: 1200 })
            .add_message::<LevelUpMsg>()
            .add_systems(Startup, setup_system)
            .add_systems(
                Update,
                (add_xp_system, player_level_up_system, log_level_up_system),
            );
    }
}

#[derive(Component)]
#[require(PlayerXp, Level)]
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

#[derive(Component, Default, Deref, DerefMut)]
struct PlayerXp(u32);

#[derive(Component)]
struct Level(u32);

impl Default for Level {
    fn default() -> Self {
        Self(1)
    }
}

#[derive(Message)]
struct LevelUpMsg {
    entity: Entity,
    name: String,
    level: u32,
}

fn setup_system(mut cmd: Commands) {
    cmd.spawn_batch([
        Player::new("amiao"),
        Player::new("yahaha"),
        Player::new("yusong"),
    ])
}

fn player_level_up_system(
    game_rules: Res<GameRules>,
    mut even_writer: MessageWriter<LevelUpMsg>,
    mut query: Query<(Entity, &Player, &mut PlayerXp, &mut Level)>,
) {
    for (entity, player, mut xp, mut level) in query.iter_mut() {
        if xp.0 > game_rules.level_up_xp {
            xp.0 -= game_rules.level_up_xp;
            level.0 += 1;
            even_writer.write(LevelUpMsg {
                entity,
                name: player.name.clone(),
                level: level.0,
            });
        }
    }
}

fn log_level_up_system(mut even_reader: MessageReader<LevelUpMsg>) {
    for e in even_reader.read() {
        info!("+++ {}({:?}) levels up to lv{}", e.name, e.entity, e.level);
    }
}

#[derive(Resource)]
struct AddXpTimer(Timer);

impl Default for AddXpTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.5, TimerMode::Repeating))
    }
}

#[derive(Resource)]
struct GameRules {
    level_up_xp: u32,
}

fn add_xp_system(
    time: Res<Time>,
    game_rules: Res<GameRules>,
    mut xp_timer: ResMut<AddXpTimer>,
    mut query: Query<(&mut PlayerXp, &Player), With<Player>>,
) {
    use rand::Rng;
    let mut rng = rand::rng();

    if !xp_timer.0.tick(time.delta()).just_finished() {
        return;
    }

    for (mut xp, player) in query.iter_mut() {
        if rng.random_bool(0.5) {
            let delta_xp: u32 = rng.random_range(200..500);
            xp.0 += delta_xp;
            debug!(
                "{} add {} xp ({} / {})",
                player.name, delta_xp, xp.0, game_rules.level_up_xp
            );
        }
    }
}
