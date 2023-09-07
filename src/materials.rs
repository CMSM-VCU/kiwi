use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::prelude::*;

/// Resource used to store materials
#[derive(Resource, Serialize, Deserialize)]
pub struct Materials(pub HashMap<u32, BondBasedLinearElastic>);
impl Materials{
    /// Returns the `BondBasedLinearElastic`, given the associated `MaterialID`
    pub fn get(&self, id: &MaterialID) -> &BondBasedLinearElastic{
        self.0.get(&id.0).expect("Material {id.0} not found")
    }
}

/// Used to identify an associated material
#[derive(Component)]
pub struct MaterialID(pub u32); 
impl PartialEq for MaterialID {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for MaterialID{}

/// Trait for materials that are used in a bond-based PD simulation
pub trait BondBasedMaterial{
    /// The force a bond will apply, given the strain of the bond (may need to be altered for different size `MaterialPoints`)
    fn force(&self, strain: f32) -> f32;
}

/// Linear force response to strain
#[derive(Component, Serialize, Deserialize)]
pub struct BondBasedLinearElastic{
    /// ID of the material, 
    pub id: u32,
    
    /// Elastic modulus / Young's modulus
    pub elastic_modulus: f32,

    /// Strain at which a bond will break
    pub critical_strain: f32,
}

impl BondBasedMaterial for BondBasedLinearElastic{
    fn force(&self, strain: f32) -> f32 {
        self.elastic_modulus * strain
    }
}