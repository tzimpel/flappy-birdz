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
    pub gravity: f32,
    pub flap_velocity: f32,
    pub background_parallax_factor: f32,
    pub pipe_size: Vec2,
    pub pipe_gap_size: f32,
    pub world_scroll_speed: f32,
    pub pipe_spawn_interval: Duration,
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
            gravity: 1000.0,
            flap_velocity: 300.0,
            background_parallax_factor: 0.0005,
            pipe_size: Vec2::new(32.0, canvas_size.y),
            pipe_gap_size: 100.0,
            world_scroll_speed: 200.0,
            pipe_spawn_interval: Duration::from_millis(1000),
            score_font_size: 33.0,
            score_margin_top: 20.0,
            foreground_color: Color::srgb_u8(0x28, 0x28, 0x28),
        }
    }
}
