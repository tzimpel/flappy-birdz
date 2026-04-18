use bevy::prelude::*;

use super::messages::ScorePoint;

#[derive(Resource, Default)]
pub struct Score(pub u32);

pub fn increment_score_on_point(
    mut score_points: MessageReader<ScorePoint>,
    mut score: ResMut<Score>,
) {
    for _ in score_points.read() {
        score.0 += 1;
    }
}
