use bevy::{camera::ScalingMode, prelude::*};

use super::config::GameConfig;

pub fn spawn_camera(mut commands: Commands, config: Res<GameConfig>) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::AutoMax {
                max_width: config.canvas_size.x,
                max_height: config.canvas_size.y,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}
