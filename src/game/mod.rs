mod assets;
mod background;
mod bird;
mod camera;
mod config;
mod messages;
mod model;
mod pipes;
mod run;
mod score;
mod state;
mod ui;

use bevy::{prelude::*, sprite_render::Material2dPlugin};

use self::{
    assets::GameAssets,
    background::{BackgroundMaterial, WorldScroll},
    config::GameConfig,
    messages::{BirdDamaged, BirdDied, RunEndRequested, RunStarted, ScorePoint},
    pipes::PipeSpawnTimer,
    run::RunDirector,
    score::Score,
    state::GameState,
};

pub struct FlappyBirdPlugin;

impl Plugin for FlappyBirdPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameConfig>()
            .init_resource::<GameAssets>()
            .init_resource::<WorldScroll>()
            .init_resource::<PipeSpawnTimer>()
            .init_resource::<RunDirector>()
            .init_resource::<Score>()
            .init_state::<GameState>()
            .add_message::<BirdDamaged>()
            .add_message::<BirdDied>()
            .add_message::<RunEndRequested>()
            .add_message::<RunStarted>()
            .add_message::<ScorePoint>()
            .add_plugins(Material2dPlugin::<BackgroundMaterial>::default())
            .add_systems(
                Startup,
                (
                    camera::spawn_camera,
                    background::configure_gizmos,
                    bird::spawn_player,
                    ui::spawn_hud,
                    background::spawn_background,
                ),
            )
            .add_systems(
                OnEnter(GameState::Ready),
                (
                    run::reset_run_entities,
                    model::sync_transforms_after_reset,
                    run::start_first_run,
                )
                    .chain(),
            )
            .add_systems(
                FixedUpdate,
                (
                    bird::apply_bird_intents,
                    bird::apply_gravity,
                    bird::integrate_velocity,
                    pipes::shift_pipes_to_the_left,
                    pipes::spawn_pipes,
                    pipes::despawn_pipes,
                    model::sync_transforms,
                    bird::check_in_bounds,
                    bird::check_collisions,
                    bird::apply_bird_damage,
                    bird::detect_bird_death,
                    run::request_run_end_on_bird_death,
                    score::increment_score_on_point,
                )
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                (
                    bird::capture_player_input,
                    background::update_parallax_offsets,
                    background::sync_parallax_materials,
                    bird::sync_bird_rotation,
                )
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                (
                    ui::score_update.run_if(resource_changed::<Score>),
                    ui::health_update,
                    model::sync_transforms.run_if(in_state(GameState::Playing)),
                    run::begin_playing_on_run_started,
                    run::finish_run_on_request.run_if(in_state(GameState::Playing)),
                ),
            )
            .add_systems(
                OnEnter(GameState::GameOver),
                run::queue_next_run_from_game_over,
            );
    }
}
