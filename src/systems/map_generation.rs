use bevy::prelude::*;
use crate::components::tile::Tile;

pub fn generate_map(mut commands: Commands) {
    for x in 0..10 {
        for y in 0..10 {
            commands.spawn(SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(x as f32 * 32.0, y as f32 * 32.0, 0.0),
                    ..default()
                },
                ..default()
            })
            .insert(Tile { x, y });
        }
    }
}
