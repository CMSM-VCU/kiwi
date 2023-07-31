
// Import node module
mod node;
// Bring everything from it's namespace to this one
pub use node::*;


// pub struct SimulationParameters{
//     // Formulation dependant
//     pub grid_spacing: f64,

//     // Formulation dependant
//     pub horizon: f64,

//     // Timeintegration dependant
//     pub delta_time: f64
// }