fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerBehaviorPlugin)
        .run();
}
