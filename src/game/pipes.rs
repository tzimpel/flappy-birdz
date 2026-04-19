use std::time::Duration;

use bevy::prelude::*;

use super::{
    assets::GameAssets,
    config::GameConfig,
    messages::ScorePoint,
    model::{
        Collider, Pipe, PipeBottom, PipeOwner, PipeResolution, PipeTop, PlayerControlled, Position,
    },
    run::DifficultyDirector,
};

#[derive(Resource)]
pub struct ObstacleDirector {
    pub time_until_spawn: f32,
    pub last_gap_center_y: f32,
    pub step_pattern_index: usize,
}

impl FromWorld for ObstacleDirector {
    fn from_world(world: &mut World) -> Self {
        let interval = world.resource::<GameConfig>().pipe_spawn_interval_easy;
        Self {
            time_until_spawn: interval.as_secs_f32(),
            last_gap_center_y: 0.0,
            step_pattern_index: 0,
        }
    }
}

pub struct CurrentObstacleParams {
    pub spawn_interval: Duration,
    pub gap_size: f32,
    pub gap_center_range: f32,
    pub gap_center_step_limit: f32,
    pub gap_step_pattern_scale: f32,
}

const GAP_STEP_PATTERN: [f32; 8] = [0.7, -0.25, 0.45, -0.6, 0.2, -0.75, 0.55, -0.3];

pub fn spawn_initial_pipe_for_run(
    commands: &mut Commands,
    config: &GameConfig,
    assets: &GameAssets,
) {
    let spawn_position = initial_pipe_spawn_position(config.canvas_size.x, config.pipe_size.x);
    spawn_pipe(
        commands,
        config,
        assets,
        spawn_position,
        0.0,
        current_pipe_gap_size(config, 0.0),
    );
}

pub fn spawn_pipes(
    mut commands: Commands,
    time: Res<Time>,
    mut obstacle_director: ResMut<ObstacleDirector>,
    config: Res<GameConfig>,
    assets: Res<GameAssets>,
    difficulty: Res<DifficultyDirector>,
) {
    obstacle_director.time_until_spawn -= time.delta_secs();

    while obstacle_director.time_until_spawn <= 0.0 {
        let params = current_obstacle_params(&config, difficulty.normalized);
        let (gap_center_y, next_pattern_index) = next_gap_center_y(
            obstacle_director.last_gap_center_y,
            obstacle_director.step_pattern_index,
            &params,
        );
        let spawn_position = repeating_pipe_spawn_position(config.canvas_size.x);
        spawn_pipe(
            &mut commands,
            &config,
            &assets,
            spawn_position,
            gap_center_y,
            params.gap_size,
        );
        obstacle_director.last_gap_center_y = gap_center_y;
        obstacle_director.step_pattern_index = next_pattern_index;
        obstacle_director.time_until_spawn += params.spawn_interval.as_secs_f32();
    }
}

pub fn shift_pipes_to_the_left(
    mut pipes: Query<&mut Position, With<Pipe>>,
    time: Res<Time>,
    config: Res<GameConfig>,
) {
    for mut pipe in &mut pipes {
        pipe.0.x -= config.world_scroll_speed * time.delta_secs();
    }
}

pub fn despawn_pipes(
    mut commands: Commands,
    pipes: Query<(Entity, &Position), With<Pipe>>,
    config: Res<GameConfig>,
) {
    for (entity, position) in pipes.iter() {
        if position.0.x < -(config.canvas_size.x / 2.0 + config.pipe_size.x) {
            commands.entity(entity).despawn();
        }
    }
}

fn spawn_pipe(
    commands: &mut Commands,
    config: &GameConfig,
    assets: &GameAssets,
    spawn_position: Vec2,
    gap_y_position: f32,
    gap_size: f32,
) {
    let image_mode = SpriteImageMode::Sliced(TextureSlicer {
        border: BorderRect::axes(8.0, 19.0),
        center_scale_mode: SliceScaleMode::Stretch,
        ..default()
    });
    let pipe_offset = config.pipe_size.y / 2.0 + gap_size / 2.0;
    let root = commands
        .spawn((
            Pipe,
            PipeResolution::Unresolved,
            Position(spawn_position),
            Transform::from_xyz(spawn_position.x, spawn_position.y, 1.0),
            Visibility::Visible,
        ))
        .id();

    commands.entity(root).with_children(|parent| {
        parent.spawn((
            PipeOwner(root),
            Collider::Rect(config.pipe_size),
            Sprite {
                image: assets.pipe_image.clone(),
                custom_size: Some(config.pipe_size),
                image_mode: image_mode.clone(),
                ..default()
            },
            Transform::from_xyz(0.0, pipe_offset + gap_y_position, 1.0),
            PipeTop,
        ));
        parent.spawn((
            PipeOwner(root),
            Collider::Rect(config.pipe_size),
            Sprite {
                image: assets.pipe_image.clone(),
                custom_size: Some(config.pipe_size),
                image_mode,
                ..default()
            },
            Transform::from_xyz(0.0, -pipe_offset + gap_y_position, 1.0),
            PipeBottom,
        ));
    });
}

pub fn initial_pipe_spawn_position(canvas_width: f32, pipe_width: f32) -> Vec2 {
    Vec2::new(canvas_width / 2.0 - pipe_width / 2.0, 0.0)
}

pub fn repeating_pipe_spawn_position(canvas_width: f32) -> Vec2 {
    Vec2::new(canvas_width / 2.0, 0.0)
}

pub fn lerp_f32(start: f32, end: f32, t: f32) -> f32 {
    start + (end - start) * t
}

pub fn lerp_duration(start: Duration, end: Duration, t: f32) -> Duration {
    Duration::from_secs_f32(lerp_f32(start.as_secs_f32(), end.as_secs_f32(), t))
}

pub fn current_pipe_spawn_interval(config: &GameConfig, difficulty: f32) -> Duration {
    lerp_duration(
        config.pipe_spawn_interval_easy,
        config.pipe_spawn_interval_hard,
        difficulty,
    )
}

pub fn current_pipe_gap_size(config: &GameConfig, difficulty: f32) -> f32 {
    lerp_f32(
        config.pipe_gap_size_easy,
        config.pipe_gap_size_hard,
        difficulty,
    )
}

pub fn current_pipe_gap_center_range(config: &GameConfig, difficulty: f32) -> f32 {
    lerp_f32(
        config.pipe_gap_center_range_easy,
        config.pipe_gap_center_range_hard,
        difficulty,
    )
}

pub fn current_pipe_gap_center_step_limit(config: &GameConfig, difficulty: f32) -> f32 {
    lerp_f32(
        config.pipe_gap_center_step_limit_easy,
        config.pipe_gap_center_step_limit_hard,
        difficulty,
    )
}

pub fn current_pipe_gap_step_pattern_scale(config: &GameConfig, difficulty: f32) -> f32 {
    lerp_f32(
        config.pipe_gap_step_pattern_scale_easy,
        config.pipe_gap_step_pattern_scale_hard,
        difficulty,
    )
}

pub fn current_obstacle_params(config: &GameConfig, difficulty: f32) -> CurrentObstacleParams {
    CurrentObstacleParams {
        spawn_interval: current_pipe_spawn_interval(config, difficulty),
        gap_size: current_pipe_gap_size(config, difficulty),
        gap_center_range: current_pipe_gap_center_range(config, difficulty),
        gap_center_step_limit: current_pipe_gap_center_step_limit(config, difficulty),
        gap_step_pattern_scale: current_pipe_gap_step_pattern_scale(config, difficulty),
    }
}

pub fn next_gap_center_y(
    last_gap_center_y: f32,
    step_pattern_index: usize,
    params: &CurrentObstacleParams,
) -> (f32, usize) {
    let step_multiplier = GAP_STEP_PATTERN[step_pattern_index % GAP_STEP_PATTERN.len()];
    let candidate = last_gap_center_y
        + step_multiplier * params.gap_step_pattern_scale * params.gap_center_step_limit;
    let next_pattern_index = (step_pattern_index + 1) % GAP_STEP_PATTERN.len();

    if candidate > params.gap_center_range {
        (params.gap_center_range, next_pattern_index)
    } else if candidate < -params.gap_center_range {
        (-params.gap_center_range, next_pattern_index)
    } else {
        (candidate, next_pattern_index)
    }
}

pub fn score_safe_pipe_passes(
    player: Single<(&Position, &Collider), With<PlayerControlled>>,
    mut pipes: Query<(&Position, &mut PipeResolution), With<Pipe>>,
    mut score_points: MessageWriter<ScorePoint>,
    config: Res<GameConfig>,
) {
    let bird_left_x = bird_left_edge(player.0.0.x, player.1);

    for (position, mut resolution) in &mut pipes {
        if *resolution != PipeResolution::Unresolved {
            continue;
        }

        let pipe_right_x = pipe_right_edge(position.0.x, config.pipe_size.x);
        if has_bird_safely_passed_pipe(bird_left_x, pipe_right_x) {
            score_points.write(ScorePoint);
            *resolution = PipeResolution::Scored;
        }
    }
}

pub fn pipe_right_edge(pipe_x: f32, pipe_width: f32) -> f32 {
    pipe_x + pipe_width / 2.0
}

pub fn bird_left_edge(bird_x: f32, collider: &Collider) -> f32 {
    let half_width = match collider {
        Collider::Circle(radius) => *radius,
        Collider::Rect(size) => size.x / 2.0,
    };

    bird_x - half_width
}

pub fn has_bird_safely_passed_pipe(bird_left_x: f32, pipe_right_x: f32) -> bool {
    bird_left_x > pipe_right_x
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use bevy::prelude::*;

    use super::{
        CurrentObstacleParams, bird_left_edge, current_obstacle_params,
        current_pipe_gap_center_range, current_pipe_gap_center_step_limit, current_pipe_gap_size,
        current_pipe_gap_step_pattern_scale, current_pipe_spawn_interval,
        has_bird_safely_passed_pipe, initial_pipe_spawn_position, lerp_duration, lerp_f32,
        next_gap_center_y, pipe_right_edge, repeating_pipe_spawn_position,
    };

    fn test_config() -> crate::game::config::GameConfig {
        crate::game::config::GameConfig::default()
    }

    #[test]
    fn initial_pipe_starts_fully_visible_on_right_edge() {
        assert_eq!(
            initial_pipe_spawn_position(480.0, 32.0),
            Vec2::new(224.0, 0.0)
        );
    }

    #[test]
    fn repeating_pipe_spawns_at_right_boundary_center() {
        assert_eq!(repeating_pipe_spawn_position(480.0), Vec2::new(240.0, 0.0));
    }

    #[test]
    fn pipe_right_edge_uses_half_width_offset() {
        assert_eq!(pipe_right_edge(224.0, 32.0), 240.0);
    }

    #[test]
    fn bird_left_edge_uses_circle_radius() {
        assert_eq!(bird_left_edge(10.0, &super::Collider::Circle(5.0)), 5.0);
    }

    #[test]
    fn bird_left_edge_uses_half_rect_width() {
        assert_eq!(
            bird_left_edge(10.0, &super::Collider::Rect(Vec2::new(8.0, 12.0))),
            6.0
        );
    }

    #[test]
    fn bird_must_be_strictly_past_pipe_to_score() {
        assert!(!has_bird_safely_passed_pipe(240.0, 240.0));
    }

    #[test]
    fn bird_scores_once_fully_past_pipe() {
        assert!(has_bird_safely_passed_pipe(240.1, 240.0));
    }

    #[test]
    fn lerp_f32_returns_endpoints() {
        assert_eq!(lerp_f32(10.0, 20.0, 0.0), 10.0);
        assert_eq!(lerp_f32(10.0, 20.0, 1.0), 20.0);
    }

    #[test]
    fn lerp_duration_returns_endpoints() {
        assert_eq!(
            lerp_duration(Duration::from_secs(2), Duration::from_secs(1), 0.0),
            Duration::from_secs(2)
        );
        assert_eq!(
            lerp_duration(Duration::from_secs(2), Duration::from_secs(1), 1.0),
            Duration::from_secs(1)
        );
    }

    #[test]
    fn current_spawn_interval_uses_easy_value_at_zero_difficulty() {
        let config = test_config();
        assert!(
            (current_pipe_spawn_interval(&config, 0.0).as_secs_f32()
                - config.pipe_spawn_interval_easy.as_secs_f32())
            .abs()
                < 0.0001
        );
    }

    #[test]
    fn current_spawn_interval_uses_hard_value_at_max_difficulty() {
        let config = test_config();
        assert!(
            (current_pipe_spawn_interval(&config, 1.0).as_secs_f32()
                - config.pipe_spawn_interval_hard.as_secs_f32())
            .abs()
                < 0.0001
        );
    }

    #[test]
    fn current_gap_size_uses_easy_value_at_zero_difficulty() {
        let config = test_config();
        assert_eq!(
            current_pipe_gap_size(&config, 0.0),
            config.pipe_gap_size_easy
        );
    }

    #[test]
    fn current_gap_size_uses_hard_value_at_max_difficulty() {
        let config = test_config();
        assert_eq!(
            current_pipe_gap_size(&config, 1.0),
            config.pipe_gap_size_hard
        );
    }

    #[test]
    fn current_gap_center_range_uses_easy_value_at_zero_difficulty() {
        let config = test_config();
        assert_eq!(
            current_pipe_gap_center_range(&config, 0.0),
            config.pipe_gap_center_range_easy
        );
    }

    #[test]
    fn current_gap_center_range_uses_hard_value_at_max_difficulty() {
        let config = test_config();
        assert_eq!(
            current_pipe_gap_center_range(&config, 1.0),
            config.pipe_gap_center_range_hard
        );
    }

    #[test]
    fn current_gap_center_step_limit_uses_easy_value_at_zero_difficulty() {
        let config = test_config();
        assert_eq!(
            current_pipe_gap_center_step_limit(&config, 0.0),
            config.pipe_gap_center_step_limit_easy
        );
    }

    #[test]
    fn current_gap_center_step_limit_uses_hard_value_at_max_difficulty() {
        let config = test_config();
        assert_eq!(
            current_pipe_gap_center_step_limit(&config, 1.0),
            config.pipe_gap_center_step_limit_hard
        );
    }

    #[test]
    fn current_gap_step_pattern_scale_uses_easy_value_at_zero_difficulty() {
        let config = test_config();
        assert_eq!(
            current_pipe_gap_step_pattern_scale(&config, 0.0),
            config.pipe_gap_step_pattern_scale_easy
        );
    }

    #[test]
    fn current_gap_step_pattern_scale_uses_hard_value_at_max_difficulty() {
        let config = test_config();
        assert_eq!(
            current_pipe_gap_step_pattern_scale(&config, 1.0),
            config.pipe_gap_step_pattern_scale_hard
        );
    }

    #[test]
    fn current_obstacle_params_combines_expected_easy_values() {
        let config = test_config();
        let params = current_obstacle_params(&config, 0.0);

        assert_eq!(params.gap_size, config.pipe_gap_size_easy);
        assert_eq!(params.gap_center_range, config.pipe_gap_center_range_easy);
        assert_eq!(
            params.gap_center_step_limit,
            config.pipe_gap_center_step_limit_easy
        );
        assert_eq!(
            params.gap_step_pattern_scale,
            config.pipe_gap_step_pattern_scale_easy
        );
    }

    #[test]
    fn next_gap_center_advances_inside_range() {
        let params = CurrentObstacleParams {
            spawn_interval: Duration::from_secs(1),
            gap_size: 100.0,
            gap_center_range: 40.0,
            gap_center_step_limit: 18.0,
            gap_step_pattern_scale: 1.0,
        };

        assert_eq!(next_gap_center_y(0.0, 0, &params), (12.599999, 1));
    }

    #[test]
    fn next_gap_center_clamps_and_flips_at_top_bound() {
        let params = CurrentObstacleParams {
            spawn_interval: Duration::from_secs(1),
            gap_size: 100.0,
            gap_center_range: 40.0,
            gap_center_step_limit: 18.0,
            gap_step_pattern_scale: 1.0,
        };

        assert_eq!(next_gap_center_y(30.0, 0, &params), (40.0, 1));
    }

    #[test]
    fn next_gap_center_clamps_and_flips_at_bottom_bound() {
        let params = CurrentObstacleParams {
            spawn_interval: Duration::from_secs(1),
            gap_size: 100.0,
            gap_center_range: 40.0,
            gap_center_step_limit: 18.0,
            gap_step_pattern_scale: 1.0,
        };

        assert_eq!(next_gap_center_y(-30.0, 5, &params), (-40.0, 6));
    }

    #[test]
    fn next_gap_center_uses_signed_patterned_step_multiplier() {
        let params = CurrentObstacleParams {
            spawn_interval: Duration::from_secs(1),
            gap_size: 100.0,
            gap_center_range: 40.0,
            gap_center_step_limit: 20.0,
            gap_step_pattern_scale: 1.0,
        };

        assert_eq!(next_gap_center_y(0.0, 2, &params), (9.0, 3));
    }

    #[test]
    fn next_gap_center_scales_pattern_step_with_difficulty_multiplier() {
        let params = CurrentObstacleParams {
            spawn_interval: Duration::from_secs(1),
            gap_size: 100.0,
            gap_center_range: 40.0,
            gap_center_step_limit: 20.0,
            gap_step_pattern_scale: 1.25,
        };

        assert_eq!(next_gap_center_y(0.0, 2, &params), (11.25, 3));
    }
}
