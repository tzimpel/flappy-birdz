use bevy::prelude::*;

use super::{
    config::GameConfig,
    messages::RunRestartRequested,
    model::{BirdIntent, PlayerControlled, Position, Velocity},
    score::Score,
};

pub fn restart_run(
    mut restarts: MessageReader<RunRestartRequested>,
    mut player: Single<(&mut Position, &mut Velocity, &mut BirdIntent), With<PlayerControlled>>,
    mut score: ResMut<Score>,
    config: Res<GameConfig>,
) {
    if restarts.read().next().is_none() {
        return;
    }

    score.0 = 0;
    player.0.0 = restart_position(config.canvas_size.x);
    player.1.0 = Vec2::ZERO;
    player.2.flap = false;
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
