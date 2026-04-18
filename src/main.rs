use bevy::prelude::*;
use flappy_birdz::FlappyBirdPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, FlappyBirdPlugin))
        .run();
}
