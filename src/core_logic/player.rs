// src/core_logic/player.rs

use bevy::prelude::*;
use bevy::app::App;
use bevy::DefaultPlugins;

// 玩家組件，包含玩家的基本屬性和狀態
#[derive(Component)]
struct Player {
    speed: f32,
    resources_collected: u32,
}

// 玩家行為系統，處理玩家的移動邏輯
fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
    time: Res<Time>,
) {
    for (player, mut transform) in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::W) {
            direction.z -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::S) {
            direction.z += 1.0;
        }
        if keyboard_input.pressed(KeyCode::A) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::D) {
            direction.x += 1.0;
        }

        let movement = direction.normalize_or_zero() * player.speed * time.delta_seconds();
        transform.translation += movement;
    }
}

// 玩家資源收集系統
fn player_collect_resource_system(
    keyboard_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    mut query: Query<&mut Player>,
    resource_query: Query<&Transform, With<ResourceNode>>,
    player_transform: Query<&Transform, With<Player>>,
) {
    if keyboard_input.just_pressed(KeyCode::E) || mouse_input.just_pressed(MouseButton::Left) {
        let player_pos = player_transform.single().translation;
        for resource_transform in resource_query.iter() {
            let resource_pos = resource_transform.translation;
            if player_pos.distance(resource_pos) < 2.0 {
                // 假設玩家成功收集資源
                let mut player = query.single_mut();
                player.resources_collected += 1;
                println!("Player collected a resource! Total: {}", player.resources_collected);
            }
        }
    }
}

// 資源節點組件，用於標記可以被玩家收集的資源
#[derive(Component)]
struct ResourceNode;

// 插件，用於添加玩家行為管理系統到 Bevy 的應用中
pub struct PlayerBehaviorPlugin;

impl Plugin for PlayerBehaviorPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_players)
            .add_system(player_movement_system)
            .add_system(player_collect_resource_system);
    }
}

// 初始化玩家
fn setup_players(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    // 初始化玩家
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
        sprite: Sprite::new(Vec2::new(50.0, 50.0)),
        ..Default::default()
    })
    .insert(Player {
        speed: 5.0,
        resources_collected: 0,
    })
    .insert(Transform::default())
    .insert(GlobalTransform::default());
    
    // 初始化資源節點
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
        sprite: Sprite::new(Vec2::new(30.0, 30.0)),
        transform: Transform {
            translation: Vec3::new(3.0, 0.0, 3.0),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(ResourceNode)
    .insert(GlobalTransform::default());
}