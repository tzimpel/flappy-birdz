mod hazards;
mod health;
mod input;
mod movement;
mod spawn;

pub use hazards::{
    check_collisions, clamp_bird_to_vertical_bounds_and_emit_impact_damage,
    damage_birds_touching_vertical_bounds,
};
pub use health::{
    apply_bird_damage, apply_passive_healing, detect_bird_death, track_recent_damage,
};
pub use input::capture_player_input;
pub use movement::{apply_bird_intents, apply_gravity, integrate_velocity, sync_bird_rotation};
pub use spawn::spawn_player;
