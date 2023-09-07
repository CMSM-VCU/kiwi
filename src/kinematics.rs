use serde::{Serialize, Deserialize};
use kd_tree::KdPoint;

use crate::prelude::*;

/// The spatial location of something in a reference configuration
#[derive(Component, Default, Serialize, Deserialize, Debug, PartialEq)]
pub struct Position(pub Vec3A);
impl KdPoint for Position{
    type Scalar = f32;
    type Dim = typenum::U2;
    fn at(&self, k: usize) -> f32 { self.0[k] }
}
impl KdPoint for &Position{
    type Scalar = f32;
    type Dim = typenum::U2;
    fn at(&self, k: usize) -> f32 { self.0[k] }
}

/// The change in spatial location of somthing from it's reference configuration
#[derive(Component, Default, Serialize, Deserialize)]
pub struct Displacement(pub Vec3A);

/// The change in spatial location with respect to time
#[derive(Component, Default, Serialize, Deserialize)]
pub struct Velocity(pub Vec3A);

/// The force vector experienced by a point mass
#[derive(Component, Default, Serialize, Deserialize)]
pub struct Force(pub Vec3A);

/// Mass of a point mass
#[derive(Component, Default, Serialize, Deserialize)]
pub struct Mass(pub f32);
