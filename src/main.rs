use bevy::prelude::*;
use tokio::runtime::Runtime;
use user_management::auth::AuthManager;
use user_management::user::User;
use components::player::Player;
use systems;
use network;

mod components;
mod resources;
mod utils;
mod user_management;
mod network;

fn main() {
    // 創建一個新的 Runtime 用於啟動異步服務器
    let rt = Runtime::new().unwrap();
    rt.spawn(network::server::run_server()); // 啟動服務器

    // 用戶管理部分 - 註冊和登錄
    let mut auth_manager = AuthManager::new();
    auth_manager.register("player1", "password123");

    let mut current_user: Option<User> = None;

    if auth_manager.login("player1", "password123") {
        println!("登入成功！");
        current_user = Some(User::new("player1"));
        if let Some(user) = &mut current_user {
            user.gain_experience(120);
            println!("{:?}", user);
        }
    } else {
        println!("登入失敗！");
    }

    // Bevy 應用部分
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(current_user) // 將玩家資料存入資源中
        .add_startup_system(setup.system())
        .add_startup_system(systems::map_generation::generate_map)
        .add_startup_system(systems::player_movement::spawn_player)
        .add_startup_system(systems::enemy_ai::spawn_enemy)
        .add_system(systems::player_movement::player_movement)
        .add_system(systems::enemy_ai::enemy_follow_player)
        .run();
}

fn setup(mut commands: Commands, user: Res<Option<User>>) {
    // 如果有玩家登錄，創建一個玩家實體
    if let Some(user) = &*user {
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.5, 0.5, 1.0),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 1.0),
                    ..default()
                },
                ..default()
            })
            .insert(Player)
            .insert(user.clone()); // 將用戶的部分信息賦予玩家實體
    }
}
