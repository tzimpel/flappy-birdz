use bevy::prelude::*;

use super::{
    assets::GameAssets,
    config::GameConfig,
    messages::{BirdDied, RunEndRequested, RunStarted},
    model::{
        Alive, BirdIntent, Health, MaxHealth, Pipe, PlayerControlled, Position, TimeSinceDamage,
        Velocity,
    },
    pipes::{PipeSpawnTimer, spawn_initial_pipe_for_run},
    score::Score,
    state::GameState,
};

#[derive(Resource, Default)]
pub struct RunDirector {
    pub current_run: u32,
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
    mut spawn_timer: ResMut<PipeSpawnTimer>,
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
    spawn_timer.0 = Timer::new(config.pipe_spawn_interval, TimerMode::Repeating);
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

pub fn queue_next_run_from_game_over(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Ready);
}

pub fn restart_position(canvas_width: f32) -> Vec2 {
    Vec2::new(-canvas_width / 4.0, 0.0)
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;

    use super::restart_position;

    #[test]
    fn restart_position_places_player_on_left_quarter() {
        assert_eq!(restart_position(480.0), Vec2::new(-120.0, 0.0));
    }
}
