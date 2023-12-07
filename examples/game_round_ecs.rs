// #![cfg_attr(debug_assertions, allow(unused))]

use bevy::prelude::*;

fn main() {
    use bevy::app::ScheduleRunnerPlugin;
    use std::time::Duration;

    println!("\n#### game_round_ecs ####\n");

    App::new()
        .add_plugins((
            ScheduleRunnerPlugin::run_loop(Duration::from_secs(1)),
            MyGamePlugin,
        ))
        .run();
}

struct MyGamePlugin;

impl Plugin for MyGamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameRules>()
            .init_resource::<GameState>()
            .add_systems(Startup, (init_player_system, log_game_rule_system))
            .configure_sets(
                Update,
                (
                    MySystemSet::BeforeRound,
                    MySystemSet::Round,
                    MySystemSet::AfterRound,
                )
                    .chain(),
            )
            .add_systems(Update, game_round_system.in_set(MySystemSet::BeforeRound))
            .add_systems(Update, add_player_system.after(MySystemSet::BeforeRound))
            .add_systems(
                Update,
                (
                    add_score_system,
                    (check_score_system, log_score_system).after(add_score_system),
                )
                    .in_set(MySystemSet::Round),
            )
            .add_systems(Update, game_over_system.in_set(MySystemSet::AfterRound))
            .add_systems(Last, print_at_last);
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum MySystemSet {
    BeforeRound,
    Round,
    AfterRound,
}

#[derive(Component)]
struct Player;

#[derive(Component, Clone)]
struct Name(String);

#[derive(Component, Default, Deref, DerefMut)]
struct Score(usize);

#[derive(Resource, Default)]
struct GameState {
    current_round: usize,
    total_players: usize,
    winner: Option<Name>,
}

#[derive(Resource)]
struct GameRules {
    max_round: usize,
    max_player: usize,
    score_to_win: usize,
}

impl Default for GameRules {
    fn default() -> Self {
        Self {
            max_round: 8,
            max_player: 5,
            score_to_win: 4,
        }
    }
}

use std::fmt::{Display, Formatter};
impl Display for GameRules {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "max_round: {}, max_player: {}, score_to_win: {}",
            self.max_round, self.max_player, self.score_to_win
        )
    }
}

/// Add player at startup.
fn init_player_system(mut game_state: ResMut<GameState>, mut cmd: Commands) {
    cmd.spawn_batch([
        (Player, Name("Amiao".to_owned()), Score::default()),
        (Player, Name("Tom".to_owned()), Score::default()),
    ]);
    game_state.total_players = 2;
}

fn log_game_rule_system(game_rules: Res<GameRules>) {
    println!("Game Rules:\n{}\n", game_rules.as_ref());
}

/// Randomly score a point to players.
fn add_score_system(mut query: Query<(&mut Score, &Name), With<Player>>) {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    for (mut score, name) in &mut query {
        if rng.gen::<bool>() {
            **score += 1;
            println!("+++ {}'s score +1", name.0);
        }
    }
}

/// Log every player's score.
fn log_score_system(query: Query<(&Score, &Name), With<Player>>) {
    for (score, name) in &query {
        println!("{}'s score is {}", name.0, score.0);
    }
}

/// Check if any player reaches the winning score.
fn check_score_system(
    game_rules: Res<GameRules>,
    mut game_state: ResMut<GameState>,
    query: Query<(&Score, &Name), With<Player>>,
) {
    for (score, name) in &query {
        if score.0 >= game_rules.score_to_win {
            game_state.winner = Some((*name).clone());
        }
    }
}

/// Randomly add new palyer.
fn add_player_system(
    mut cmd: Commands,
    game_rules: Res<GameRules>,
    mut game_state: ResMut<GameState>,
) {
    if rand::random::<bool>() && game_state.total_players < game_rules.max_player {
        game_state.total_players += 1;
        let name = format!("Player {}", game_state.total_players);
        println!("{} will participate in the next round", name);
        cmd.spawn((Player, Name(name), Score::default()));
    }
}

/// Iterate game rounds.
/// Make game over if max round reaches.
fn game_round_system(mut game_state: ResMut<GameState>, game_rules: Res<GameRules>) {
    game_state.current_round += 1;
    println!(
        "Round {} of {} starts...",
        game_state.current_round, game_rules.max_round
    );
}

use bevy::app::AppExit;
/// Exit the app when game is over
fn game_over_system(
    game_state: Res<GameState>,
    game_rules: Res<GameRules>,
    mut app_exit_event: EventWriter<AppExit>,
) {
    if let Some(name) = &game_state.winner {
        println!("\nWinner is {}\n", name.0);
        app_exit_event.send(AppExit);
    } else if game_state.current_round >= game_rules.max_round {
        println!("\nGame over without a winner\n");
        app_exit_event.send(AppExit);
    }
}

/// Sometimes systems need to be stateful.\
/// Bevy's ECS provides the `Local` system parameter for this case.
fn print_at_last(mut counter: Local<u32>) {
    *counter += 1;
    println!("---- {} ----\n", *counter);
}
