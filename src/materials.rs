use bevy::prelude::*;


#[derive(Component)]
pub struct Material;

#[derive(Component)]
pub struct MaterialID(pub u32); 


trait BondBasedMaterial{
    fn force(strain: f32) -> f32;
    
}