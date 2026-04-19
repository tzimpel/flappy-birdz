use bevy::prelude::*;

use super::director::{
    ObstacleDirector, current_obstacle_params, current_pipe_gap_size, next_gap_center_y,
};

use crate::game::{
    assets::GameAssets,
    config::GameConfig,
    model::{Collider, Pipe, PipeBottom, PipeOwner, PipeResolution, PipeTop, Position},
    run::DifficultyDirector,
};

pub fn spawn_initial_pipe_for_run(
    commands: &mut Commands,
    config: &GameConfig,
    assets: &GameAssets,
) {
    let spawn_position = initial_pipe_spawn_position(config.canvas_size.x, config.pipe_size.x);
    spawn_pipe(
        commands,
        config,
        assets,
        spawn_position,
        0.0,
        current_pipe_gap_size(config, 0.0),
    );
}

pub fn spawn_pipes(
    mut commands: Commands,
    time: Res<Time>,
    mut obstacle_director: ResMut<ObstacleDirector>,
    config: Res<GameConfig>,
    assets: Res<GameAssets>,
    difficulty: Res<DifficultyDirector>,
) {
    obstacle_director.time_until_spawn -= time.delta_secs();

    while obstacle_director.time_until_spawn <= 0.0 {
        let params = current_obstacle_params(&config, difficulty.normalized);
        let (gap_center_y, next_rng_state) = next_gap_center_y(
            obstacle_director.last_gap_center_y,
            obstacle_director.rng_state,
            &params,
            &config,
        );
        let spawn_position = repeating_pipe_spawn_position(config.canvas_size.x);
        spawn_pipe(
            &mut commands,
            &config,
            &assets,
            spawn_position,
            gap_center_y,
            params.gap_size,
        );
        obstacle_director.last_gap_center_y = gap_center_y;
        obstacle_director.rng_state = next_rng_state;
        obstacle_director.time_until_spawn += params.spawn_interval.as_secs_f32();
    }
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
    gap_size: f32,
) {
    let image_mode = SpriteImageMode::Sliced(TextureSlicer {
        border: BorderRect::axes(8.0, 19.0),
        center_scale_mode: SliceScaleMode::Stretch,
        ..default()
    });
    let pipe_offset = config.pipe_size.y / 2.0 + gap_size / 2.0;
    let root = commands
        .spawn((
            Pipe,
            PipeResolution::Unresolved,
            Position(spawn_position),
            Transform::from_xyz(spawn_position.x, spawn_position.y, 1.0),
            Visibility::Visible,
        ))
        .id();

    commands.entity(root).with_children(|parent| {
        parent.spawn((
            PipeOwner(root),
            Collider::Rect(config.pipe_size),
            Sprite {
                image: assets.pipe_image.clone(),
                custom_size: Some(config.pipe_size),
                image_mode: image_mode.clone(),
                ..default()
            },
            Transform::from_xyz(0.0, pipe_offset + gap_y_position, 1.0),
            PipeTop,
        ));
        parent.spawn((
            PipeOwner(root),
            Collider::Rect(config.pipe_size),
            Sprite {
                image: assets.pipe_image.clone(),
                custom_size: Some(config.pipe_size),
                image_mode,
                ..default()
            },
            Transform::from_xyz(0.0, -pipe_offset + gap_y_position, 1.0),
            PipeBottom,
        ));
    });
}

pub fn initial_pipe_spawn_position(canvas_width: f32, pipe_width: f32) -> Vec2 {
    Vec2::new(canvas_width / 2.0 - pipe_width / 2.0, 0.0)
}

pub fn repeating_pipe_spawn_position(canvas_width: f32) -> Vec2 {
    Vec2::new(canvas_width / 2.0, 0.0)
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;

    use super::{initial_pipe_spawn_position, repeating_pipe_spawn_position};

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
}
