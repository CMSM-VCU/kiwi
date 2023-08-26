
#[derive(Component)]
pub struct Material;


trait BondBasedMaterial{
    fn force(strain: f32) -> f32;
    
}