use bevy::prelude::*;

#[derive(Message)]
pub struct RunEndRequested;

#[derive(Message)]
pub struct RunStarted;

#[derive(Message)]
pub struct ScorePoint;
