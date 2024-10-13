use bevy::prelude::*;

mod components;
mod systems;
mod resources;
mod utils;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(systems::map_generation::generate_map)
        .add_startup_system(systems::player_movement::spawn_player)
        .add_startup_system(systems::enemy_ai::spawn_enemy)
        .add_system(systems::player_movement::player_movement)
        .add_system(systems::enemy_ai::enemy_follow_player)
        .run();
}
