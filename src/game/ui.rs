use bevy::prelude::*;

use super::{
    config::GameConfig,
    model::{Health, MaxHealth, PlayerControlled},
    score::Score,
};

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct HealthText;

pub fn spawn_hud(mut commands: Commands, config: Res<GameConfig>) {
    commands.spawn((
        Node {
            width: percent(100.0),
            margin: px(config.score_margin_top).top(),
            justify_content: JustifyContent::Center,
            column_gap: px(24.0),
            ..default()
        },
        children![
            (
                Text::new("Score 0"),
                TextLayout::new_with_justify(Justify::Center),
                TextFont {
                    font_size: config.score_font_size,
                    ..default()
                },
                TextColor(config.foreground_color),
                ScoreText,
            ),
            (
                Text::new(format!(
                    "HP {} / {}",
                    config.bird_max_health.round() as i32,
                    config.bird_max_health.round() as i32
                )),
                TextLayout::new_with_justify(Justify::Center),
                TextFont {
                    font_size: config.score_font_size,
                    ..default()
                },
                TextColor(config.foreground_color),
                HealthText,
            )
        ],
    ));
}

pub fn score_update(mut query: Query<&mut Text, With<ScoreText>>, score: Res<Score>) {
    for mut span in &mut query {
        span.0 = format!("Score {}", score.0);
    }
}

pub fn health_update(
    mut query: Query<&mut Text, With<HealthText>>,
    player: Single<(&Health, &MaxHealth), With<PlayerControlled>>,
) {
    let current = player.0.0.round() as i32;
    let max = player.1.0.round() as i32;

    for mut span in &mut query {
        span.0 = format!("HP {current} / {max}");
    }
}
