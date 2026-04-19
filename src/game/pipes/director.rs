use std::time::Duration;

use bevy::prelude::*;

use crate::game::config::GameConfig;

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

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::{
        CurrentObstacleParams, current_obstacle_params, current_pipe_gap_center_range,
        current_pipe_gap_center_step_limit, current_pipe_gap_size,
        current_pipe_gap_step_pattern_scale, current_pipe_spawn_interval, lerp_duration, lerp_f32,
        next_gap_center_y,
    };

    fn test_config() -> crate::game::config::GameConfig {
        crate::game::config::GameConfig::default()
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
