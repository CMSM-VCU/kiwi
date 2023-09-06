use bevy::{prelude::*, utils::HashMap};
use serde::{Serialize, Deserialize};

use crate::prelude::*;

use kd_tree::{KdTree, KdPoint};


/// Unit struct for labeling entities as a `Bond`
#[derive(Component)]
pub struct Bond;

/// Populates the strain component of `Bond`s
#[allow(clippy::needless_pass_by_value)]
pub fn calc_bond_strain(
    mut bonds: Query<(&mut Strain, &Connection), With<Bond>>,
    nodes: Query<(Entity, &Position, &Displacement), With<MaterialPoint>>
){
    for (mut strain, connection) in bonds.iter_mut(){
        let from = nodes.get(connection.from).expect("");
        let to = nodes.get(connection.to).expect("");

        let initial_length = from.1.0.distance(to.1.0);
        let current_length = (from.1.0 + from.2.0).distance(to.1.0 + to.2.0);

        strain.0 = (initial_length + current_length) / initial_length;
    }
}


/// Struct for creating bonds from the reference configuration
#[derive(PartialEq)]
struct ReferencePoint<'a> (Entity, &'a Position);
impl KdPoint for ReferencePoint<'_>{
    type Scalar = f32;
    type Dim = typenum::U2;
    fn at(&self, k: usize) -> f32 { self.1.0[k] }
}

/// Consumes horizon value from `InputFile`. This may need to change in the future as some formulations require that info
/// Creates `Bond`s with `MaterialPoint`s in a spherical radius, horizon, around them. Bonds are spawned as separate `Entity`s
#[allow(clippy::needless_pass_by_value)]
pub fn create_reference_bonds_spherical(
    material_points: Query<(Entity, &Position), With<MaterialPoint>>,
    mut config: ResMut<InputFile>,
    mut commands: Commands
){
    #[allow(clippy::cast_possible_truncation)]
    let horizon: f32 = config.get("horizon").expect("key 'horizon' not found in config file").as_float().expect("'horizon' is not a float") as f32;

    let tree = KdTree::build_by_ordered_float(material_points.iter().map(|x|{ReferencePoint(x.0, x.1)}).collect());

    let mut connections: HashMap<u64, Connection> = HashMap::new();

    for (entity, position) in material_points.iter(){
        let neighbor_positions: Vec<&ReferencePoint> = tree.within_radius(position, horizon);
        for neighbor_position in neighbor_positions{
            if neighbor_position.1 != position{
                let other = material_points.get(neighbor_position.0).expect("Problem finding neighbor node").0;
                let connection = Connection{
                    from: entity,
                    to: other
                };

                let key: u64 = u64::from(entity.index())*u64::from(other.index());

                if !connections.contains_key(&key){
                    connections.insert(key, connection);
                }
            }
        }
    }
    
    // Maybe instead of a bond entity, each material point could have a vec of enities (good for state-based, bad for bond-based)
    for connection in connections.into_iter().map(|(_key, val)| val ){
        trace!("Creating bond: ({:?}, {:?})", connection.from.index(), connection.to.index());
        commands.spawn((
            Bond,
            connection,
        ));
    }
}




/// Represents a connection between two entites, used in a `Bond` to identify what it's connected to
/// 
#[derive(Component, Serialize, Deserialize)]
pub struct Connection{
    /// One side of the connection
    pub from: Entity,
    /// The other side of the connection
    pub to: Entity
}
impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        (self.from == other.from && self.to == other.to) || (self.to == other.from && self.from == other.to)
    }
}
impl Eq for Connection{}

/// Strain component for a bond in a bond-based PD simulation
#[derive(Component, Default, Serialize, Deserialize)]
pub struct Strain(pub f32);
