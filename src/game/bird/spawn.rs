use bevy::prelude::*;

use crate::game::{
    assets::GameAssets,
    config::GameConfig,
    model::{
        Alive, Bird, Collider, Gravity, Health, MaxHealth, PlayerControlled, Position, RegenRate,
        TimeSinceDamage,
    },
};

pub fn spawn_player(mut commands: Commands, config: Res<GameConfig>, assets: Res<GameAssets>) {
    commands.spawn((
        Alive,
        Bird,
        PlayerControlled,
        Gravity(config.gravity),
        Health(config.bird_max_health),
        MaxHealth(config.bird_max_health),
        RegenRate(config.bird_regen_rate),
        TimeSinceDamage(config.bird_regen_delay_secs),
        Position(Vec2::new(-config.canvas_size.x / 4.0, 0.0)),
        Collider::Circle(config.player_size / 2.0),
        Sprite {
            custom_size: Some(Vec2::splat(config.player_size)),
            image: assets.bird_image.clone(),
            color: config.foreground_color,
            ..default()
        },
        Transform::from_xyz(-config.canvas_size.x / 4.0, 0.0, 1.0),
    ));
}
