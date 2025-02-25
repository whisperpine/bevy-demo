// #![cfg_attr(debug_assertions, allow(unused))]

use bevy::{ecs::system::SystemParam, prelude::*};

fn main() {
    App::new()
        .insert_resource(PlayerNumber(0))
        .add_systems(Startup, add_players)
        .add_systems(Update, count_players)
        .run();
}

#[derive(Component)]
struct Player;

fn add_players(mut cmd: Commands) {
    cmd.spawn_batch([Player, Player, Player]);
}

#[derive(Resource)]
struct PlayerNumber(usize);

#[derive(SystemParam)]
struct PlayerCounter<'w, 's> {
    res: ResMut<'w, PlayerNumber>,
    query: Query<'w, 's, (), With<Player>>,
}

impl PlayerCounter<'_, '_> {
    fn count(&mut self) {
        self.res.0 = self.query.iter().len();
    }
}

fn count_players(mut player_counter: PlayerCounter) {
    player_counter.count();
    println!("player number: {}", player_counter.res.0);
}
