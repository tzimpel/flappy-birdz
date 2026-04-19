use bevy::prelude::*;

use crate::game::{
    config::GameConfig,
    messages::{BirdDamaged, BirdDied},
    model::{Alive, Bird, Health, MaxHealth, RegenRate, TimeSinceDamage},
};

pub fn apply_bird_damage(
    mut bird_damaged: MessageReader<BirdDamaged>,
    mut birds: Query<&mut Health, With<Bird>>,
) {
    for damage in bird_damaged.read() {
        if let Ok(mut health) = birds.get_mut(damage.entity) {
            health.0 = (health.0 - damage.amount).max(0.0);
        }
    }
}

pub fn track_recent_damage(
    mut bird_damaged: MessageReader<BirdDamaged>,
    mut birds: Query<&mut TimeSinceDamage, With<Bird>>,
) {
    for damage in bird_damaged.read() {
        if let Ok(mut time_since_damage) = birds.get_mut(damage.entity) {
            time_since_damage.0 = 0.0;
        }
    }
}

pub fn detect_bird_death(
    mut commands: Commands,
    mut bird_died: MessageWriter<BirdDied>,
    birds: Query<(Entity, &Health), (With<Bird>, With<Alive>)>,
) {
    for (entity, health) in &birds {
        if health.0 <= 0.0 {
            bird_died.write(BirdDied { entity });
            commands.entity(entity).remove::<Alive>();
        }
    }
}

pub fn apply_passive_healing(
    mut birds: Query<
        (&mut Health, &MaxHealth, &RegenRate, &mut TimeSinceDamage),
        (With<Bird>, With<Alive>),
    >,
    time: Res<Time>,
    config: Res<GameConfig>,
) {
    for (mut health, max_health, regen_rate, mut time_since_damage) in &mut birds {
        time_since_damage.0 += time.delta_secs();

        if time_since_damage.0 >= config.bird_regen_delay_secs {
            health.0 = (health.0 + regen_rate.0 * time.delta_secs()).min(max_health.0);
        }
    }
}
