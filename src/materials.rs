use bevy::prelude::*;

/// Defines the mechanical behavior of a `MaterialPoint` 
#[derive(Component)]
pub struct Material;

/// Used to identify an associated material
#[derive(Component)]
pub struct MaterialID(pub u32); 


trait BondBasedMaterial{
    fn force(strain: f32) -> f32;
    
}