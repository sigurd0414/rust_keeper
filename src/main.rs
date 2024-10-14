use bevy::prelude::*;
use core_logic::player::PlayerBehaviorPlugin;

fn main() {
    App::new() // 使用 App::new() 而不是 App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerBehaviorPlugin)
        .run();
}
