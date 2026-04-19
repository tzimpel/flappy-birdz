use bevy::prelude::*;

#[derive(Message)]
pub struct RunEndRequested;

#[derive(Message)]
pub struct BirdDamaged {
    pub entity: Entity,
    pub amount: f32,
}

#[derive(Message)]
pub struct RunStarted;

#[derive(Message)]
pub struct ScorePoint;
