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
    messages::{RunRestartRequested, ScorePoint},
    pipes::PipeSpawnTimer,
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
            .init_resource::<Score>()
            .init_state::<GameState>()
            .add_message::<RunRestartRequested>()
            .add_message::<ScorePoint>()
            .add_plugins(Material2dPlugin::<BackgroundMaterial>::default())
            .add_systems(
                Startup,
                (
                    camera::spawn_camera,
                    background::configure_gizmos,
                    bird::spawn_player,
                    pipes::spawn_initial_pipe,
                    ui::spawn_score_ui,
                    background::spawn_background,
                ),
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
                    score::increment_score_on_point,
                    run::restart_run,
                    model::sync_transforms_after_reset,
                )
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                (
                    bird::capture_player_input,
                    ui::score_update.run_if(resource_changed::<Score>),
                    background::update_parallax_offsets,
                    background::sync_parallax_materials,
                    bird::sync_bird_rotation,
                )
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                model::sync_transforms.run_if(in_state(GameState::Playing)),
            );
    }
}
