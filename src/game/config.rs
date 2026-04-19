use std::time::Duration;

use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct GameConfig {
    pub canvas_size: Vec2,
    pub player_size: f32,
    pub bird_max_health: f32,
    pub bird_regen_rate: f32,
    pub bird_regen_delay_secs: f32,
    pub pipe_collision_damage: f32,
    pub boundary_impact_damage_scale: f32,
    pub boundary_impact_damage_max: f32,
    pub boundary_contact_damage_per_second: f32,
    pub difficulty_ramp_duration_secs: f32,
    pub game_over_delay_secs: f32,
    pub gravity: f32,
    pub flap_velocity: f32,
    pub background_parallax_factor: f32,
    pub pipe_size: Vec2,
    pub pipe_gap_size_easy: f32,
    pub pipe_gap_size_hard: f32,
    pub pipe_gap_center_range_easy: f32,
    pub pipe_gap_center_range_hard: f32,
    pub pipe_gap_center_step_limit_easy: f32,
    pub pipe_gap_center_step_limit_hard: f32,
    pub pipe_gap_step_pattern_scale_easy: f32,
    pub pipe_gap_step_pattern_scale_hard: f32,
    pub world_scroll_speed: f32,
    pub pipe_spawn_interval_easy: Duration,
    pub pipe_spawn_interval_hard: Duration,
    pub score_font_size: f32,
    pub score_margin_top: f32,
    pub foreground_color: Color,
}

impl Default for GameConfig {
    fn default() -> Self {
        let canvas_size = Vec2::new(480.0, 270.0);

        Self {
            canvas_size,
            player_size: 25.0,
            bird_max_health: 100.0,
            bird_regen_rate: 6.0,
            bird_regen_delay_secs: 2.0,
            pipe_collision_damage: 25.0,
            boundary_impact_damage_scale: 0.05,
            boundary_impact_damage_max: 20.0,
            boundary_contact_damage_per_second: 8.0,
            difficulty_ramp_duration_secs: 45.0,
            game_over_delay_secs: 1.25,
            gravity: 1000.0,
            flap_velocity: 300.0,
            background_parallax_factor: 0.0005,
            pipe_size: Vec2::new(32.0, canvas_size.y),
            pipe_gap_size_easy: 110.0,
            pipe_gap_size_hard: 80.0,
            pipe_gap_center_range_easy: 40.0,
            pipe_gap_center_range_hard: 85.0,
            pipe_gap_center_step_limit_easy: 18.0,
            pipe_gap_center_step_limit_hard: 40.0,
            pipe_gap_step_pattern_scale_easy: 0.7,
            pipe_gap_step_pattern_scale_hard: 1.3,
            world_scroll_speed: 200.0,
            pipe_spawn_interval_easy: Duration::from_millis(1200),
            pipe_spawn_interval_hard: Duration::from_millis(700),
            score_font_size: 33.0,
            score_margin_top: 20.0,
            foreground_color: Color::srgb_u8(0x28, 0x28, 0x28),
        }
    }
}
