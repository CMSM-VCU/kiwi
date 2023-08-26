use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use bevy::math::Vec3A;

#[derive(Bundle)]
pub struct KinematicBundle {
    pub position: Position,
    pub displacement: Displacement,
    pub velocity: Velocity,
    pub force: Force,
    pub mass: Mass
}

#[derive(Component, Default, Serialize, Deserialize)]
pub struct Position(pub Vec3A);

#[derive(Component, Default, Serialize, Deserialize)]
pub struct Displacement(pub Vec3A);

#[derive(Component, Default, Serialize, Deserialize)]
pub struct Velocity(pub Vec3A);

#[derive(Component, Default, Serialize, Deserialize)]
pub struct Force(pub Vec3A);

#[derive(Component, Default, Serialize, Deserialize)]
pub struct Mass(pub f32);
