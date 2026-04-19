use bevy::prelude::*;
use std::time::Duration;

use super::{
    assets::GameAssets,
    config::GameConfig,
    messages::{BirdDied, RunEndRequested, RunStarted},
    model::{
        Alive, BirdIntent, Health, MaxHealth, Pipe, PlayerControlled, Position, TimeSinceDamage,
        Velocity,
    },
    pipes::{ObstacleDirector, spawn_initial_pipe_for_run},
    score::Score,
    state::GameState,
};

#[derive(Resource, Default)]
pub struct RunDirector {
    pub current_run: u32,
}

#[derive(Resource, Default)]
pub struct DifficultyDirector {
    pub elapsed_secs: f32,
    pub normalized: f32,
}

#[derive(Resource, Default)]
pub struct GameOverDelayTimer {
    pub timer: Timer,
}

pub fn start_first_run(mut run_started: MessageWriter<RunStarted>) {
    run_started.write(RunStarted);
}

pub fn reset_run_entities(
    mut player: Single<
        (
            Entity,
            &mut Position,
            &mut Velocity,
            &mut BirdIntent,
            &mut Health,
            &MaxHealth,
            &mut TimeSinceDamage,
        ),
        With<PlayerControlled>,
    >,
    mut score: ResMut<Score>,
    mut difficulty: ResMut<DifficultyDirector>,
    mut obstacle_director: ResMut<ObstacleDirector>,
    mut fixed_time: ResMut<Time<Fixed>>,
    pipes: Query<Entity, With<Pipe>>,
    assets: Res<GameAssets>,
    mut commands: Commands,
    config: Res<GameConfig>,
) {
    score.0 = 0;
    player.1.0 = restart_position(config.canvas_size.x);
    player.2.0 = Vec2::ZERO;
    player.3.flap = false;
    player.4.0 = player.5.0;
    player.6.0 = config.bird_regen_delay_secs;
    difficulty.elapsed_secs = 0.0;
    difficulty.normalized = 0.0;
    obstacle_director.time_until_spawn = config.pipe_spawn_interval_easy.as_secs_f32();
    obstacle_director.last_gap_center_y = 0.0;
    obstacle_director.step_pattern_index = 0;
    let fixed_overstep = fixed_time.overstep();
    fixed_time.discard_overstep(fixed_overstep);
    commands.entity(player.0).insert(Alive);

    for entity in &pipes {
        commands.entity(entity).despawn();
    }

    spawn_initial_pipe_for_run(&mut commands, &config, &assets);
}

pub fn begin_playing_on_run_started(
    mut run_started: MessageReader<RunStarted>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if run_started.read().next().is_some() {
        next_state.set(GameState::Playing);
    }
}

pub fn advance_run_difficulty(
    mut difficulty: ResMut<DifficultyDirector>,
    time: Res<Time>,
    config: Res<GameConfig>,
) {
    difficulty.elapsed_secs += time.delta_secs();
    difficulty.normalized = normalized_difficulty(
        difficulty.elapsed_secs,
        config.difficulty_ramp_duration_secs,
    );
}

pub fn finish_run_on_request(
    mut run_end_requests: MessageReader<RunEndRequested>,
    mut next_state: ResMut<NextState<GameState>>,
    mut run_director: ResMut<RunDirector>,
) {
    if run_end_requests.read().next().is_none() {
        return;
    }

    run_director.current_run += 1;
    next_state.set(GameState::GameOver);
}

pub fn request_run_end_on_bird_death(
    mut bird_died: MessageReader<BirdDied>,
    mut run_end_requests: MessageWriter<RunEndRequested>,
) {
    if let Some(death) = bird_died.read().next() {
        let _entity = death.entity;
        run_end_requests.write(RunEndRequested);
    }
}

pub fn begin_game_over_delay(
    mut game_over_delay: ResMut<GameOverDelayTimer>,
    config: Res<GameConfig>,
) {
    game_over_delay.timer = Timer::new(
        Duration::from_secs_f32(config.game_over_delay_secs),
        TimerMode::Once,
    );
}

pub fn advance_game_over_delay(
    mut game_over_delay: ResMut<GameOverDelayTimer>,
    mut next_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
) {
    if game_over_delay.timer.tick(time.delta()).just_finished() {
        next_state.set(GameState::Ready);
    }
}

pub fn restart_position(canvas_width: f32) -> Vec2 {
    Vec2::new(-canvas_width / 4.0, 0.0)
}

pub fn normalized_difficulty(elapsed_secs: f32, ramp_duration_secs: f32) -> f32 {
    (elapsed_secs / ramp_duration_secs).clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;

    use super::{normalized_difficulty, restart_position};

    #[test]
    fn restart_position_places_player_on_left_quarter() {
        assert_eq!(restart_position(480.0), Vec2::new(-120.0, 0.0));
    }

    #[test]
    fn normalized_difficulty_starts_at_zero() {
        assert_eq!(normalized_difficulty(0.0, 45.0), 0.0);
    }

    #[test]
    fn normalized_difficulty_reaches_halfway_mid_ramp() {
        assert_eq!(normalized_difficulty(22.5, 45.0), 0.5);
    }

    #[test]
    fn normalized_difficulty_caps_at_one() {
        assert_eq!(normalized_difficulty(60.0, 45.0), 1.0);
    }
}
