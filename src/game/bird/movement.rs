use bevy::prelude::*;

use crate::game::{
    config::GameConfig,
    model::{Bird, BirdIntent, Gravity, PlayerControlled, Position, Velocity},
};

pub fn apply_bird_intents(
    mut birds: Query<(&mut Velocity, &mut BirdIntent), With<PlayerControlled>>,
    config: Res<GameConfig>,
) {
    for (mut velocity, mut intent) in &mut birds {
        if intent.flap {
            velocity.0.y = config.flap_velocity;
            intent.flap = false;
        }
    }
}

pub fn apply_gravity(mut birds: Query<(&mut Velocity, &Gravity), With<Bird>>, time: Res<Time>) {
    for (mut velocity, gravity) in &mut birds {
        velocity.0.y -= gravity.0 * time.delta_secs();
    }
}

pub fn integrate_velocity(mut movers: Query<(&mut Position, &Velocity)>, time: Res<Time>) {
    for (mut position, velocity) in &mut movers {
        position.0 += velocity.0 * time.delta_secs();
    }
}

pub fn sync_bird_rotation(
    mut birds: Query<(&mut Transform, &Velocity), With<Bird>>,
    config: Res<GameConfig>,
) {
    for (mut transform, velocity) in &mut birds {
        let facing_vector = Vec2::new(config.world_scroll_speed, velocity.0.y);
        transform.rotation = Quat::from_rotation_z(facing_vector.to_angle());
    }
}
