use bevy::prelude::*;

use crate::game::model::{BirdIntent, PlayerControlled};

pub fn capture_player_input(
    mut birds: Query<&mut BirdIntent, With<PlayerControlled>>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    if !buttons.any_just_pressed([MouseButton::Left, MouseButton::Right]) {
        return;
    }

    for mut intent in &mut birds {
        intent.flap = true;
    }
}
