#![warn(clippy::all, clippy::pedantic, missing_docs)]
//! Safe peridynamic crate for rust, focusing on modularity, ease of modification, user ease, and performance, in that order.

mod parse;
mod kinematics;
mod material_point;
mod bond;
mod materials;

/// Easy access to public types across the crate
pub mod prelude{
    pub use crate::parse::*;
    pub use crate::kinematics::*;
    pub use crate::material_point::*;
    pub use crate::bond::*;
    pub use crate::materials::*;
}

pub use prelude::*;
