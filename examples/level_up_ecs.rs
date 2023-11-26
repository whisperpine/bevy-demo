#![cfg_attr(debug_assertions, allow(unused))]

use bevy::{log::LogPlugin, prelude::*};

fn main() {
    println!("\n#### level_up_ecs ####\n");
    const CRATE_NAME: &str = env!("CARGO_CRATE_NAME");

    App::new()
        .add_plugins((
            MinimalPlugins,
            LogPlugin {
                filter: format!("wgpu=error,naga=warn,{}=debug", CRATE_NAME),
                ..Default::default()
            },
            MyGamePlugin,
        ))
        .run();
}

struct MyGamePlugin;

impl Plugin for MyGamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AddXpTimer>()
            .insert_resource(GameRules { level_up_xp: 1200 })
            .add_event::<LevelUpEvent>()
            .add_systems(Startup, setup_system)
            .add_systems(
                Update,
                (add_xp_system, player_level_up_system, log_level_up_system),
            );
    }
}

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    name: Name,
    level: Level,
    xp: PlayerXp,
}

impl PlayerBundle {
    fn new(name: &str) -> Self {
        Self {
            player: Player,
            name: Name(name.to_owned()),
            level: Level(1),
            xp: PlayerXp(0),
        }
    }
}

#[derive(Component)]
struct Player;

#[derive(Component, Deref, DerefMut)]
struct PlayerXp(u32);

#[derive(Component, Deref)]
struct Name(String);

#[derive(Component)]
struct Level(u32);

#[derive(Event)]
struct LevelUpEvent {
    entity: Entity,
    name: String,
    level: u32,
}

fn setup_system(mut cmd: Commands) {
    cmd.spawn_batch([
        PlayerBundle::new("amiao"),
        PlayerBundle::new("yahaha"),
        PlayerBundle::new("yusong"),
    ])
}

fn player_level_up_system(
    game_rules: Res<GameRules>,
    mut even_writer: EventWriter<LevelUpEvent>,
    mut query: Query<(Entity, &Name, &mut PlayerXp, &mut Level), With<Player>>,
) {
    for (entity, name, mut xp, mut level) in query.iter_mut() {
        if xp.0 > game_rules.level_up_xp {
            xp.0 -= game_rules.level_up_xp;
            level.0 += 1;
            even_writer.send(LevelUpEvent {
                entity,
                name: name.0.clone(),
                level: level.0,
            })
        }
    }
}

fn log_level_up_system(mut even_reader: EventReader<LevelUpEvent>) {
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
    mut query: Query<(&mut PlayerXp, &Name), With<Player>>,
) {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    if !xp_timer.0.tick(time.delta()).just_finished() {
        return;
    }

    for (mut xp, name) in query.iter_mut() {
        if rng.gen_bool(0.5) {
            let delta_xp: u32 = rng.gen_range(200..500);
            xp.0 += delta_xp;
            debug!(
                "{} add {} xp ({} / {})",
                name.0, delta_xp, xp.0, game_rules.level_up_xp
            );
        }
    }
}
