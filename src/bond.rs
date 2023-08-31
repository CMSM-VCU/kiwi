use bevy::prelude::*;
use serde::{Serialize, Deserialize};

use crate::prelude::*;

#[derive(Component, Serialize, Deserialize)]
pub struct Bond;



fn calc_bond_strain(
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


// Connection
#[derive(Component, Serialize, Deserialize)]
struct Connection{
    pub from: Entity,
    pub to: Entity
}
impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        return (self.from == other.from && self.to == other.to) || (self.to == other.from && self.from == other.to)
    }
}
impl Eq for Connection{}

// Strain
#[derive(Component, Default, Serialize, Deserialize)]
struct Strain(pub f32);
