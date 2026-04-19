use bevy::prelude::*;

use super::{
    assets::GameAssets,
    config::GameConfig,
    model::{Collider, Pipe, PipeBottom, PipeTop, PointsGate, Position},
};

#[derive(Resource)]
pub struct PipeSpawnTimer(pub Timer);

impl FromWorld for PipeSpawnTimer {
    fn from_world(world: &mut World) -> Self {
        let interval = world.resource::<GameConfig>().pipe_spawn_interval;
        Self(Timer::new(interval, TimerMode::Repeating))
    }
}

pub fn spawn_initial_pipe_for_run(
    commands: &mut Commands,
    config: &GameConfig,
    assets: &GameAssets,
) {
    let spawn_position = initial_pipe_spawn_position(config.canvas_size.x, config.pipe_size.x);
    spawn_pipe(commands, config, assets, spawn_position, 0.0);
}

pub fn spawn_pipes(
    mut commands: Commands,
    time: Res<Time>,
    mut spawn_timer: ResMut<PipeSpawnTimer>,
    config: Res<GameConfig>,
    assets: Res<GameAssets>,
) {
    if !spawn_timer.0.tick(time.delta()).just_finished() {
        return;
    }

    let spawn_position = repeating_pipe_spawn_position(config.canvas_size.x);
    let gap_y_position = compute_gap_y_position(time.elapsed_secs(), config.canvas_size.y);
    spawn_pipe(
        &mut commands,
        &config,
        &assets,
        spawn_position,
        gap_y_position,
    );
}

pub fn shift_pipes_to_the_left(
    mut pipes: Query<&mut Position, With<Pipe>>,
    time: Res<Time>,
    config: Res<GameConfig>,
) {
    for mut pipe in &mut pipes {
        pipe.0.x -= config.world_scroll_speed * time.delta_secs();
    }
}

pub fn despawn_pipes(
    mut commands: Commands,
    pipes: Query<(Entity, &Position), With<Pipe>>,
    config: Res<GameConfig>,
) {
    for (entity, position) in pipes.iter() {
        if position.0.x < -(config.canvas_size.x / 2.0 + config.pipe_size.x) {
            commands.entity(entity).despawn();
        }
    }
}

fn spawn_pipe(
    commands: &mut Commands,
    config: &GameConfig,
    assets: &GameAssets,
    spawn_position: Vec2,
    gap_y_position: f32,
) {
    let image_mode = SpriteImageMode::Sliced(TextureSlicer {
        border: BorderRect::axes(8.0, 19.0),
        center_scale_mode: SliceScaleMode::Stretch,
        ..default()
    });
    let pipe_offset = config.pipe_size.y / 2.0 + config.pipe_gap_size / 2.0;

    commands.spawn((
        Pipe,
        Position(spawn_position),
        Transform::from_xyz(spawn_position.x, spawn_position.y, 1.0),
        Visibility::Visible,
        children![
            (
                Collider::Rect(config.pipe_size),
                Sprite {
                    image: assets.pipe_image.clone(),
                    custom_size: Some(config.pipe_size),
                    image_mode: image_mode.clone(),
                    ..default()
                },
                Transform::from_xyz(0.0, pipe_offset + gap_y_position, 1.0,),
                PipeTop
            ),
            (
                Collider::Rect(Vec2::new(config.score_gate_width, config.pipe_gap_size,)),
                Visibility::Hidden,
                Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::new(config.score_gate_width, config.pipe_gap_size,)),
                    ..default()
                },
                Transform::from_xyz(0.0, gap_y_position, 1.0,),
                PointsGate,
            ),
            (
                Collider::Rect(config.pipe_size),
                Sprite {
                    image: assets.pipe_image.clone(),
                    custom_size: Some(config.pipe_size),
                    image_mode,
                    ..default()
                },
                Transform::from_xyz(0.0, -pipe_offset + gap_y_position, 1.0,),
                PipeBottom,
            )
        ],
    ));
}

pub fn initial_pipe_spawn_position(canvas_width: f32, pipe_width: f32) -> Vec2 {
    Vec2::new(canvas_width / 2.0 - pipe_width / 2.0, 0.0)
}

pub fn repeating_pipe_spawn_position(canvas_width: f32) -> Vec2 {
    Vec2::new(canvas_width / 2.0, 0.0)
}

pub fn compute_gap_y_position(elapsed_secs: f32, canvas_height: f32) -> f32 {
    (elapsed_secs * 4.2309875).sin() * canvas_height / 4.0
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;

    use super::{
        compute_gap_y_position, initial_pipe_spawn_position, repeating_pipe_spawn_position,
    };

    #[test]
    fn initial_pipe_starts_fully_visible_on_right_edge() {
        assert_eq!(
            initial_pipe_spawn_position(480.0, 32.0),
            Vec2::new(224.0, 0.0)
        );
    }

    #[test]
    fn repeating_pipe_spawns_at_right_boundary_center() {
        assert_eq!(repeating_pipe_spawn_position(480.0), Vec2::new(240.0, 0.0));
    }

    #[test]
    fn gap_position_starts_centered_at_time_zero() {
        assert_eq!(compute_gap_y_position(0.0, 270.0), 0.0);
    }
}
