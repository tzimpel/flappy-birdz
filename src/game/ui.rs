use bevy::prelude::*;

use super::{
    config::GameConfig,
    model::{Health, MaxHealth, PlayerControlled},
    score::Score,
    state::GameState,
};

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct HealthText;

#[derive(Component)]
pub struct GameOverText;

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
            ),
            (
                Text::new("Game Over"),
                Node {
                    position_type: PositionType::Absolute,
                    top: px(72.0),
                    width: percent(100.0),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                TextLayout::new_with_justify(Justify::Center),
                TextFont {
                    font_size: config.score_font_size,
                    ..default()
                },
                TextColor(config.foreground_color),
                Visibility::Hidden,
                GameOverText,
            ),
        ],
    ));
}

pub fn score_update(mut query: Query<&mut Text, With<ScoreText>>, score: Res<Score>) {
    for mut span in &mut query {
        span.0 = format_score_text(score.0);
    }
}

pub fn health_update(
    mut query: Query<&mut Text, With<HealthText>>,
    player: Single<(&Health, &MaxHealth), With<PlayerControlled>>,
) {
    let current = player.0.0.round() as i32;
    let max = player.1.0.round() as i32;

    for mut span in &mut query {
        span.0 = format_health_text(current, max);
    }
}

pub fn show_game_over_feedback(mut query: Query<&mut Visibility, With<GameOverText>>) {
    for mut visibility in &mut query {
        *visibility = Visibility::Visible;
    }
}

pub fn hide_game_over_feedback(mut query: Query<&mut Visibility, With<GameOverText>>) {
    for mut visibility in &mut query {
        *visibility = Visibility::Hidden;
    }
}

pub fn sync_game_over_feedback_to_state(
    mut query: Query<&mut Visibility, With<GameOverText>>,
    game_state: Res<State<GameState>>,
) {
    let visibility = if *game_state.get() == GameState::GameOver {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };

    for mut current_visibility in &mut query {
        *current_visibility = visibility;
    }
}

fn format_score_text(score: u32) -> String {
    format!("Score {score}")
}

fn format_health_text(current: i32, max: i32) -> String {
    format!("HP {current} / {max}")
}

#[cfg(test)]
mod tests {
    use super::{format_health_text, format_score_text};

    #[test]
    fn score_text_formats_current_score() {
        assert_eq!(format_score_text(7), "Score 7");
    }

    #[test]
    fn health_text_formats_current_and_max_health() {
        assert_eq!(format_health_text(42, 100), "HP 42 / 100");
    }
}
