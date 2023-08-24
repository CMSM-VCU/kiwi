//==============================
//        Components
//==============================
use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use bevy::math::Vec3A;

#[derive(Component)]
pub struct Node;

#[derive(Component, Default, Serialize, Deserialize)]
pub struct Position(pub Vec3A);

#[derive(Component, Default, Serialize, Deserialize)]
pub struct Displacement(pub Vec3A);

#[derive(Component, Default, Serialize, Deserialize)]
pub struct Velocity(pub Vec3A);

#[derive(Component, Default, Serialize, Deserialize)]
pub struct Force(pub Vec3A);

#[derive(Component, Default, Serialize, Deserialize)]
pub struct Neighbors(pub Vec<Entity>);


