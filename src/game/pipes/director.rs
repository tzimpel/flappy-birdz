use std::time::Duration;

use bevy::prelude::*;

use crate::game::config::GameConfig;

#[derive(Resource)]
pub struct ObstacleDirector {
    pub time_until_spawn: f32,
    pub last_gap_center_y: f32,
    pub rng_state: u64,
}

impl FromWorld for ObstacleDirector {
    fn from_world(world: &mut World) -> Self {
        let interval = world.resource::<GameConfig>().pipe_spawn_interval_easy;
        Self {
            time_until_spawn: interval.as_secs_f32(),
            last_gap_center_y: 0.0,
            rng_state: 0xC0FFEE_u64,
        }
    }
}

pub struct CurrentObstacleParams {
    pub spawn_interval: Duration,
    pub gap_size: f32,
    pub gap_center_range: f32,
    pub max_gap_center_jump: f32,
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

pub fn current_pipe_gap_center_jump(config: &GameConfig, difficulty: f32) -> f32 {
    lerp_f32(
        config.pipe_gap_center_jump_easy,
        config.pipe_gap_center_jump_hard,
        difficulty,
    )
}

pub fn gap_jump_cadence_factor(
    spawn_interval: Duration,
    easy_spawn_interval: Duration,
    hard_spawn_interval: Duration,
    damp_min: f32,
) -> f32 {
    if easy_spawn_interval == hard_spawn_interval {
        return 1.0;
    }

    let progress = ((easy_spawn_interval.as_secs_f32() - spawn_interval.as_secs_f32())
        / (easy_spawn_interval.as_secs_f32() - hard_spawn_interval.as_secs_f32()))
    .clamp(0.0, 1.0);

    lerp_f32(1.0, damp_min, progress)
}

pub fn current_obstacle_params(config: &GameConfig, difficulty: f32) -> CurrentObstacleParams {
    CurrentObstacleParams {
        spawn_interval: current_pipe_spawn_interval(config, difficulty),
        gap_size: current_pipe_gap_size(config, difficulty),
        gap_center_range: current_pipe_gap_center_range(config, difficulty),
        max_gap_center_jump: current_pipe_gap_center_jump(config, difficulty),
    }
}

pub fn next_rng_state(rng_state: u64) -> u64 {
    // xorshift64* gives a compact deterministic sequence without extra dependencies.
    let mut x = rng_state.max(1);
    x ^= x >> 12;
    x ^= x << 25;
    x ^= x >> 27;
    x.wrapping_mul(0x2545_F491_4F6C_DD1D)
}

pub fn sample_unit_f32(rng_state: u64) -> f32 {
    let bits = (rng_state >> 40) as u32;
    bits as f32 / ((1_u32 << 24) - 1) as f32
}

pub fn cadence_adjusted_max_gap_jump(params: &CurrentObstacleParams, config: &GameConfig) -> f32 {
    params.max_gap_center_jump
        * gap_jump_cadence_factor(
            params.spawn_interval,
            config.pipe_spawn_interval_easy,
            config.pipe_spawn_interval_hard,
            config.pipe_gap_jump_cadence_damp_min,
        )
}

pub fn next_gap_center_y(
    last_gap_center_y: f32,
    rng_state: u64,
    params: &CurrentObstacleParams,
    config: &GameConfig,
) -> (f32, u64) {
    let next_rng_state = next_rng_state(rng_state);
    let sample = sample_unit_f32(next_rng_state);
    let sampled_center = lerp_f32(-params.gap_center_range, params.gap_center_range, sample);
    let max_jump = cadence_adjusted_max_gap_jump(params, config);
    let clamped_delta = (sampled_center - last_gap_center_y).clamp(-max_jump, max_jump);
    let next_center = (last_gap_center_y + clamped_delta)
        .clamp(-params.gap_center_range, params.gap_center_range);

    (next_center, next_rng_state)
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::{
        CurrentObstacleParams, cadence_adjusted_max_gap_jump, current_obstacle_params,
        current_pipe_gap_center_jump, current_pipe_gap_center_range, current_pipe_gap_size,
        current_pipe_spawn_interval, gap_jump_cadence_factor, lerp_duration, lerp_f32,
        next_gap_center_y, next_rng_state,
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
    fn current_gap_center_jump_uses_easy_value_at_zero_difficulty() {
        let config = test_config();
        assert_eq!(
            current_pipe_gap_center_jump(&config, 0.0),
            config.pipe_gap_center_jump_easy
        );
    }

    #[test]
    fn current_gap_center_jump_uses_hard_value_at_max_difficulty() {
        let config = test_config();
        assert_eq!(
            current_pipe_gap_center_jump(&config, 1.0),
            config.pipe_gap_center_jump_hard
        );
    }

    #[test]
    fn current_obstacle_params_combines_expected_easy_values() {
        let config = test_config();
        let params = current_obstacle_params(&config, 0.0);

        assert_eq!(params.gap_size, config.pipe_gap_size_easy);
        assert_eq!(params.gap_center_range, config.pipe_gap_center_range_easy);
        assert_eq!(params.max_gap_center_jump, config.pipe_gap_center_jump_easy);
    }

    #[test]
    fn next_rng_state_is_deterministic() {
        assert_eq!(next_rng_state(1234), next_rng_state(1234));
    }

    #[test]
    fn cadence_factor_is_lower_for_shorter_spawn_interval() {
        let config = test_config();
        assert!(
            gap_jump_cadence_factor(
                config.pipe_spawn_interval_hard,
                config.pipe_spawn_interval_easy,
                config.pipe_spawn_interval_hard,
                config.pipe_gap_jump_cadence_damp_min
            ) < gap_jump_cadence_factor(
                config.pipe_spawn_interval_easy,
                config.pipe_spawn_interval_easy,
                config.pipe_spawn_interval_hard,
                config.pipe_gap_jump_cadence_damp_min
            )
        );
    }

    #[test]
    fn cadence_adjusted_jump_respects_damp_min() {
        let config = test_config();
        let params = CurrentObstacleParams {
            spawn_interval: config.pipe_spawn_interval_hard,
            gap_size: 100.0,
            gap_center_range: 40.0,
            max_gap_center_jump: 50.0,
        };

        assert_eq!(
            cadence_adjusted_max_gap_jump(&params, &config),
            50.0 * config.pipe_gap_jump_cadence_damp_min
        );
    }

    #[test]
    fn next_gap_center_stays_inside_range() {
        let config = test_config();
        let params = CurrentObstacleParams {
            spawn_interval: Duration::from_secs(1),
            gap_size: 100.0,
            gap_center_range: 40.0,
            max_gap_center_jump: 90.0,
        };

        let (next_center, _) = next_gap_center_y(0.0, 1234, &params, &config);
        assert!((-40.0..=40.0).contains(&next_center));
    }

    #[test]
    fn next_gap_center_is_deterministic_for_same_seed() {
        let config = test_config();
        let params = CurrentObstacleParams {
            spawn_interval: Duration::from_secs(1),
            gap_size: 100.0,
            gap_center_range: 40.0,
            max_gap_center_jump: 90.0,
        };

        assert_eq!(
            next_gap_center_y(0.0, 1234, &params, &config),
            next_gap_center_y(0.0, 1234, &params, &config)
        );
    }

    #[test]
    fn next_gap_center_differs_for_different_seed() {
        let config = test_config();
        let params = CurrentObstacleParams {
            spawn_interval: Duration::from_secs(1),
            gap_size: 100.0,
            gap_center_range: 40.0,
            max_gap_center_jump: 90.0,
        };

        assert_ne!(
            next_gap_center_y(0.0, 1234, &params, &config),
            next_gap_center_y(0.0, 5678, &params, &config)
        );
    }

    #[test]
    fn next_gap_center_respects_jump_cap() {
        let config = test_config();
        let params = CurrentObstacleParams {
            spawn_interval: Duration::from_secs(1),
            gap_size: 100.0,
            gap_center_range: 85.0,
            max_gap_center_jump: 20.0,
        };

        let (next_center, _) = next_gap_center_y(0.0, 1234, &params, &config);
        assert!(next_center.abs() <= 20.0);
    }
}
