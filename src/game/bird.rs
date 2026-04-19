use bevy::{
    color::palettes::tailwind::RED_400,
    math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume},
    prelude::*,
};

use super::{
    assets::GameAssets,
    config::GameConfig,
    messages::{BirdDamaged, BirdDied},
    model::{
        Alive, Bird, BirdIntent, Collider, Gravity, Health, MaxHealth, PipeBottom, PipeOwner,
        PipeResolution, PipeTop, PlayerControlled, Position, RegenRate, TimeSinceDamage, Velocity,
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

pub fn capture_player_input(
    mut birds: Query<&mut BirdIntent, With<PlayerControlled>>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    if !buttons.any_just_pressed([MouseButton::Left, MouseButton::Right]) {
        return;
    }

    for mut intent in &mut birds {
        intent.flap = true;
    }
}

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

pub fn clamp_bird_to_vertical_bounds_and_emit_impact_damage(
    mut birds: Query<(Entity, &mut Position, &mut Velocity, &Collider), With<Bird>>,
    mut bird_damaged: MessageWriter<BirdDamaged>,
    config: Res<GameConfig>,
    time: Res<Time>,
) {
    for (entity, mut position, mut velocity, collider) in &mut birds {
        let (min_y, max_y) = vertical_bounds_for_bird(config.canvas_size.y, collider);
        let previous_y = position.0.y - velocity.0.y * time.delta_secs();

        if position.0.y > max_y {
            let impact_speed = outward_top_impact_speed(velocity.0.y);
            let hit_top_this_step = crossed_top_boundary_this_step(previous_y, max_y);
            position.0.y = max_y;
            if velocity.0.y > 0.0 {
                velocity.0.y = 0.0;
            }
            if hit_top_this_step && impact_speed > 0.0 {
                bird_damaged.write(BirdDamaged {
                    entity,
                    amount: impact_damage_from_speed(
                        impact_speed,
                        config.boundary_impact_damage_scale,
                        config.boundary_impact_damage_max,
                    ),
                });
            }
        } else if position.0.y < min_y {
            let impact_speed = outward_bottom_impact_speed(velocity.0.y);
            let hit_bottom_this_step = crossed_bottom_boundary_this_step(previous_y, min_y);
            position.0.y = min_y;
            if velocity.0.y < 0.0 {
                velocity.0.y = 0.0;
            }
            if hit_bottom_this_step && impact_speed > 0.0 {
                bird_damaged.write(BirdDamaged {
                    entity,
                    amount: impact_damage_from_speed(
                        impact_speed,
                        config.boundary_impact_damage_scale,
                        config.boundary_impact_damage_max,
                    ),
                });
            }
        }
    }
}

pub fn damage_birds_touching_vertical_bounds(
    birds: Query<(Entity, &Position, &Collider), With<Bird>>,
    mut bird_damaged: MessageWriter<BirdDamaged>,
    config: Res<GameConfig>,
    time: Res<Time>,
) {
    for (entity, position, collider) in &birds {
        let (min_y, max_y) = vertical_bounds_for_bird(config.canvas_size.y, collider);
        if is_touching_vertical_boundary(position.0.y, min_y, max_y) {
            bird_damaged.write(BirdDamaged {
                entity,
                amount: config.boundary_contact_damage_per_second * time.delta_secs(),
            });
        }
    }
}

pub fn check_collisions(
    mut bird_damaged: MessageWriter<BirdDamaged>,
    player: Single<(Entity, &Position, &Collider), With<PlayerControlled>>,
    pipe_segments: Query<(&Collider, &PipeOwner, Entity), Or<(With<PipeTop>, With<PipeBottom>)>>,
    mut pipe_roots: Query<&mut PipeResolution>,
    mut gizmos: Gizmos,
    transform_helper: TransformHelper,
    config: Res<GameConfig>,
) -> Result<()> {
    let player_radius = match player.2 {
        Collider::Circle(radius) => *radius,
        Collider::Rect(size) => size.x.min(size.y) / 2.0,
    };
    let player_collider = BoundingCircle::new(player.1.0, player_radius);

    gizmos.circle_2d(player.1.0, player_radius, RED_400);

    for (collider, owner, entity) in &pipe_segments {
        let pipe_transform = transform_helper.compute_global_transform(entity)?;
        let pipe_size = match collider {
            Collider::Rect(size) => *size,
            Collider::Circle(radius) => Vec2::splat(*radius * 2.0),
        };
        let pipe_collider = Aabb2d::new(pipe_transform.translation().xy(), pipe_size / 2.0);

        gizmos.rect_2d(pipe_transform.translation().xy(), pipe_size, RED_400);

        if player_collider.intersects(&pipe_collider) {
            let Ok(mut resolution) = pipe_roots.get_mut(owner.0) else {
                continue;
            };

            if *resolution == PipeResolution::Unresolved {
                bird_damaged.write(BirdDamaged {
                    entity: player.0,
                    amount: config.pipe_collision_damage,
                });
                *resolution = PipeResolution::Hit;
            }
        }
    }

    Ok(())
}

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

pub fn sync_bird_rotation(
    mut birds: Query<(&mut Transform, &Velocity), With<Bird>>,
    config: Res<GameConfig>,
) {
    for (mut transform, velocity) in &mut birds {
        let facing_vector = Vec2::new(config.world_scroll_speed, velocity.0.y);
        transform.rotation = Quat::from_rotation_z(facing_vector.to_angle());
    }
}

pub fn vertical_bounds_for_bird(canvas_height: f32, collider: &Collider) -> (f32, f32) {
    let half_height = match collider {
        Collider::Circle(radius) => *radius,
        Collider::Rect(size) => size.y / 2.0,
    };
    let min_y = -canvas_height / 2.0 + half_height;
    let max_y = canvas_height / 2.0 - half_height;

    (min_y, max_y)
}

pub fn outward_top_impact_speed(velocity_y: f32) -> f32 {
    velocity_y.max(0.0)
}

pub fn outward_bottom_impact_speed(velocity_y: f32) -> f32 {
    (-velocity_y).max(0.0)
}

pub fn impact_damage_from_speed(speed: f32, scale: f32, max_damage: f32) -> f32 {
    (speed * scale).min(max_damage)
}

pub fn is_touching_vertical_boundary(y: f32, min_y: f32, max_y: f32) -> bool {
    const EPSILON: f32 = 0.001;

    (y - min_y).abs() <= EPSILON || (y - max_y).abs() <= EPSILON
}

pub fn crossed_top_boundary_this_step(previous_y: f32, max_y: f32) -> bool {
    previous_y < max_y
}

pub fn crossed_bottom_boundary_this_step(previous_y: f32, min_y: f32) -> bool {
    previous_y > min_y
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;

    use super::{
        crossed_bottom_boundary_this_step, crossed_top_boundary_this_step,
        impact_damage_from_speed, is_touching_vertical_boundary, outward_bottom_impact_speed,
        outward_top_impact_speed, vertical_bounds_for_bird,
    };

    #[test]
    fn circle_collider_produces_expected_vertical_bounds() {
        assert_eq!(
            vertical_bounds_for_bird(270.0, &super::Collider::Circle(12.5)),
            (-122.5, 122.5)
        );
    }

    #[test]
    fn rect_collider_produces_expected_vertical_bounds() {
        assert_eq!(
            vertical_bounds_for_bird(270.0, &super::Collider::Rect(Vec2::new(8.0, 20.0))),
            (-125.0, 125.0)
        );
    }

    #[test]
    fn top_impact_speed_ignores_downward_velocity() {
        assert_eq!(outward_top_impact_speed(-10.0), 0.0);
        assert_eq!(outward_top_impact_speed(15.0), 15.0);
    }

    #[test]
    fn bottom_impact_speed_ignores_upward_velocity() {
        assert_eq!(outward_bottom_impact_speed(10.0), 0.0);
        assert_eq!(outward_bottom_impact_speed(-15.0), 15.0);
    }

    #[test]
    fn impact_damage_is_capped() {
        assert_eq!(impact_damage_from_speed(500.0, 0.05, 20.0), 20.0);
    }

    #[test]
    fn touching_boundary_detects_min_and_max() {
        assert!(is_touching_vertical_boundary(-122.5, -122.5, 122.5));
        assert!(is_touching_vertical_boundary(122.5, -122.5, 122.5));
        assert!(!is_touching_vertical_boundary(0.0, -122.5, 122.5));
    }

    #[test]
    fn top_impact_only_counts_when_crossing_from_inside() {
        assert!(crossed_top_boundary_this_step(100.0, 122.5));
        assert!(!crossed_top_boundary_this_step(122.5, 122.5));
    }

    #[test]
    fn bottom_impact_only_counts_when_crossing_from_inside() {
        assert!(crossed_bottom_boundary_this_step(-100.0, -122.5));
        assert!(!crossed_bottom_boundary_this_step(-122.5, -122.5));
    }
}
