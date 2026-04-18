use bevy::{
    color::palettes::tailwind::RED_400,
    math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume},
    prelude::*,
};

use super::{
    assets::GameAssets,
    config::GameConfig,
    messages::{RunRestartRequested, ScorePoint},
    model::{
        Bird, BirdIntent, Collider, Gravity, PipeBottom, PipeTop, PlayerControlled, PointsGate,
        Position, Velocity,
    },
};

pub fn spawn_player(mut commands: Commands, config: Res<GameConfig>, assets: Res<GameAssets>) {
    commands.spawn((
        Bird,
        PlayerControlled,
        Gravity(config.gravity),
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

pub fn check_in_bounds(
    player: Single<&Position, With<PlayerControlled>>,
    mut restarts: MessageWriter<RunRestartRequested>,
    config: Res<GameConfig>,
) {
    if is_bird_out_of_bounds(player.0.y, config.canvas_size.y, config.player_size) {
        restarts.write(RunRestartRequested);
    }
}

pub fn check_collisions(
    mut commands: Commands,
    mut restarts: MessageWriter<RunRestartRequested>,
    mut score_points: MessageWriter<ScorePoint>,
    player: Single<(&Position, &Collider), With<PlayerControlled>>,
    pipe_segments: Query<(&Collider, Entity), Or<(With<PipeTop>, With<PipeBottom>)>>,
    pipe_gaps: Query<(&Collider, Entity), With<PointsGate>>,
    mut gizmos: Gizmos,
    transform_helper: TransformHelper,
) -> Result<()> {
    let player_radius = match player.1 {
        Collider::Circle(radius) => *radius,
        Collider::Rect(size) => size.x.min(size.y) / 2.0,
    };
    let player_collider = BoundingCircle::new(player.0.0, player_radius);

    gizmos.circle_2d(player.0.0, player_radius, RED_400);

    for (collider, entity) in &pipe_segments {
        let pipe_transform = transform_helper.compute_global_transform(entity)?;
        let pipe_size = match collider {
            Collider::Rect(size) => *size,
            Collider::Circle(radius) => Vec2::splat(*radius * 2.0),
        };
        let pipe_collider = Aabb2d::new(pipe_transform.translation().xy(), pipe_size / 2.0);

        gizmos.rect_2d(pipe_transform.translation().xy(), pipe_size, RED_400);

        if player_collider.intersects(&pipe_collider) {
            restarts.write(RunRestartRequested);
        }
    }

    for (collider, entity) in &pipe_gaps {
        let gap_transform = transform_helper.compute_global_transform(entity)?;
        let gap_size = match collider {
            Collider::Rect(size) => *size,
            Collider::Circle(radius) => Vec2::splat(*radius * 2.0),
        };
        let gap_collider = Aabb2d::new(gap_transform.translation().xy(), gap_size / 2.0);

        gizmos.rect_2d(gap_transform.translation().xy(), gap_size, RED_400);

        if player_collider.intersects(&gap_collider) {
            score_points.write(ScorePoint);
            commands.entity(entity).despawn();
        }
    }

    Ok(())
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

pub fn is_bird_out_of_bounds(bird_y: f32, canvas_height: f32, bird_size: f32) -> bool {
    bird_y < -canvas_height / 2.0 - bird_size || bird_y > canvas_height / 2.0 + bird_size
}

#[cfg(test)]
mod tests {
    use super::is_bird_out_of_bounds;

    #[test]
    fn bird_inside_bounds_is_not_out_of_bounds() {
        assert!(!is_bird_out_of_bounds(0.0, 270.0, 25.0));
    }

    #[test]
    fn bird_below_bounds_is_out_of_bounds() {
        assert!(is_bird_out_of_bounds(-200.1, 270.0, 25.0));
    }

    #[test]
    fn bird_above_bounds_is_out_of_bounds() {
        assert!(is_bird_out_of_bounds(200.1, 270.0, 25.0));
    }
}
