use bevy::prelude::*;

use crate::game::{
    config::GameConfig,
    messages::ScorePoint,
    model::{Collider, Pipe, PipeResolution, PlayerControlled, Position},
};

pub fn score_safe_pipe_passes(
    player: Single<(&Position, &Collider), With<PlayerControlled>>,
    mut pipes: Query<(&Position, &mut PipeResolution), With<Pipe>>,
    mut score_points: MessageWriter<ScorePoint>,
    config: Res<GameConfig>,
) {
    let bird_left_x = bird_left_edge(player.0.0.x, player.1);

    for (position, mut resolution) in &mut pipes {
        if *resolution != PipeResolution::Unresolved {
            continue;
        }

        let pipe_right_x = pipe_right_edge(position.0.x, config.pipe_size.x);
        if has_bird_safely_passed_pipe(bird_left_x, pipe_right_x) {
            score_points.write(ScorePoint);
            *resolution = PipeResolution::Scored;
        }
    }
}

pub fn pipe_right_edge(pipe_x: f32, pipe_width: f32) -> f32 {
    pipe_x + pipe_width / 2.0
}

pub fn bird_left_edge(bird_x: f32, collider: &Collider) -> f32 {
    let half_width = match collider {
        Collider::Circle(radius) => *radius,
        Collider::Rect(size) => size.x / 2.0,
    };

    bird_x - half_width
}

pub fn has_bird_safely_passed_pipe(bird_left_x: f32, pipe_right_x: f32) -> bool {
    bird_left_x > pipe_right_x
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;

    use super::{bird_left_edge, has_bird_safely_passed_pipe, pipe_right_edge};
    use crate::game::model::Collider;

    #[test]
    fn pipe_right_edge_uses_half_width_offset() {
        assert_eq!(pipe_right_edge(224.0, 32.0), 240.0);
    }

    #[test]
    fn bird_left_edge_uses_circle_radius() {
        assert_eq!(bird_left_edge(10.0, &Collider::Circle(5.0)), 5.0);
    }

    #[test]
    fn bird_left_edge_uses_half_rect_width() {
        assert_eq!(
            bird_left_edge(10.0, &Collider::Rect(Vec2::new(8.0, 12.0))),
            6.0
        );
    }

    #[test]
    fn bird_must_be_strictly_past_pipe_to_score() {
        assert!(!has_bird_safely_passed_pipe(240.0, 240.0));
    }

    #[test]
    fn bird_scores_once_fully_past_pipe() {
        assert!(has_bird_safely_passed_pipe(240.1, 240.0));
    }
}
