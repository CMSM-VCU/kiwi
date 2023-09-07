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

    pub use bevy::math::Vec3A;

    // Not importing everything because there is a bevy::prelude::Node that is meant for UI and not to be confused with a MaterialPoint
    pub use bevy::prelude::{
        App,
        Plugin,

        PreStartup,
        Startup,
        PostStartup,
        PreUpdate,
        Update,
        PostUpdate,

        IntoSystemConfigs,

        EventWriter,
        EventReader,
        Query,
        Local,
        Commands,
        Resource,

        Component,
        With,
        Entity,
        Res,
        ResMut,

        error,
        warn,
        info,
        debug,
        trace,
    };
}

pub use prelude::*;
