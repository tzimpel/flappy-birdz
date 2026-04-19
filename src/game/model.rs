use bevy::prelude::*;

#[derive(Component)]
#[require(Position, Velocity, BirdIntent)]
pub struct Bird;

#[derive(Component)]
pub struct Alive;

#[derive(Component)]
pub struct PlayerControlled;

#[derive(Component, Default)]
pub struct Gravity(pub f32);

#[derive(Component, Default)]
pub struct Velocity(pub Vec2);

#[derive(Component, Default)]
pub struct Position(pub Vec2);

#[derive(Component, Default)]
pub struct Health(pub f32);

#[derive(Component, Default)]
pub struct MaxHealth(pub f32);

#[derive(Component, Default)]
pub struct BirdIntent {
    pub flap: bool,
}

#[derive(Component, Clone, Copy)]
pub enum Collider {
    Circle(f32),
    Rect(Vec2),
}

#[derive(Component)]
#[require(Position)]
pub struct Pipe;

#[derive(Component)]
pub struct PipeTop;

#[derive(Component)]
pub struct PipeBottom;

#[derive(Component)]
pub struct PointsGate;

pub fn sync_transforms(mut query: Query<(&Position, &mut Transform)>) {
    for (position, mut transform) in &mut query {
        transform.translation.x = position.0.x;
        transform.translation.y = position.0.y;
    }
}

pub fn sync_transforms_after_reset(query: Query<(&Position, &mut Transform)>) {
    sync_transforms(query);
}
