use bevy::prelude::*;
use serde::{Serialize, Deserialize};

use crate::prelude::*;

use kd_tree::KdTree;

#[derive(Component, Serialize, Deserialize)]
pub struct Bond;



pub fn calc_bond_strain(
    mut bonds: Query<(&mut Strain, &Connection), With<Bond>>,
    nodes: Query<(Entity, &Position, &Displacement), With<Node>>
){
    for (mut strain, connection) in bonds.iter_mut(){
        let from = nodes.get(connection.from).expect("");
        let to = nodes.get(connection.to).expect("");

        let initial_length = from.1.0.distance(to.1.0);
        let current_length = (from.1.0 + from.2.0).distance(to.1.0 + to.2.0);

        strain.0 = (initial_length + current_length) / initial_length;
    }
}

pub fn create_reference_bonds_spherical(
    nodes: Query<&Position, With<MaterialPoint>>
){
    
    let tree = KdTree::build_by_ordered_float(nodes.iter().collect());
    dbg!(tree.items());

    todo!("For each point, find neighbors within a radius and spawn bonds via bevy commands");
}


// Connection
#[derive(Component, Serialize, Deserialize)]
pub struct Connection{
    pub from: Entity,
    pub to: Entity
}
impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        (self.from == other.from && self.to == other.to) || (self.to == other.from && self.from == other.to)
    }
}
impl Eq for Connection{}

// Strain
#[derive(Component, Default, Serialize, Deserialize)]
pub struct Strain(pub f32);
