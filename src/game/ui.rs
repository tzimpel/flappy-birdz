use bevy::prelude::*;

use super::{config::GameConfig, score::Score};

#[derive(Component)]
pub struct ScoreText;

pub fn spawn_score_ui(mut commands: Commands, config: Res<GameConfig>) {
    commands.spawn((
        Node {
            width: percent(100.0),
            margin: px(config.score_margin_top).top(),
            ..default()
        },
        Text::new("0"),
        TextLayout::new_with_justify(Justify::Center),
        TextFont {
            font_size: config.score_font_size,
            ..default()
        },
        TextColor(config.foreground_color),
        ScoreText,
    ));
}

pub fn score_update(mut query: Query<&mut Text, With<ScoreText>>, score: Res<Score>) {
    for mut span in &mut query {
        span.0 = score.0.to_string();
    }
}
