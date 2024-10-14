use bevy::prelude::*;
use core_logic::player::PlayerBehaviorPlugin;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerBehaviorPlugin)
        .run();
}
