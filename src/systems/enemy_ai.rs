use bevy::prelude::*;
use crate::components::{player::Player, enemy::Enemy};

pub fn enemy_follow_player(
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for mut enemy_transform in enemy_query.iter_mut() {
            let direction = (player_transform.translation - enemy_transform.translation).normalize();
            enemy_transform.translation += direction * 1.0;
        }
    }
}
