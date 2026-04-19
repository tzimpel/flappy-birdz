use bevy::{
    color::palettes::tailwind::RED_400,
    math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume},
    prelude::*,
};

use crate::game::{
    config::GameConfig,
    messages::BirdDamaged,
    model::{
        Bird, Collider, PipeBottom, PipeOwner, PipeResolution, PipeTop, PlayerControlled, Position,
        Velocity,
    },
};

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
    use crate::game::model::Collider;

    #[test]
    fn circle_collider_produces_expected_vertical_bounds() {
        assert_eq!(
            vertical_bounds_for_bird(270.0, &Collider::Circle(12.5)),
            (-122.5, 122.5)
        );
    }

    #[test]
    fn rect_collider_produces_expected_vertical_bounds() {
        assert_eq!(
            vertical_bounds_for_bird(270.0, &Collider::Rect(Vec2::new(8.0, 20.0))),
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
