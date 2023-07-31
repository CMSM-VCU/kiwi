// use serde::{Serialize, Deserialize};


// //==============================
// //           Plugin
// //==============================
// pub struct ExplicitTimeIntegration;

// impl Plugin for ExplicitTimeIntegration {
//     fn build(&self, app: &mut App) {
//         app.
//             add_system(explicit_time_integration
//                 .run_if(resource_exists::<ExplicitTimeIntegrationSettings>()))
//     }
// }




// //==============================
// //          Resources
// //==============================

// // Time
// // Timestep
// // Timestep size

// #[derive(Resource, Serialize, Deserialize)]
// struct ExplicitTimeIntegrationSettings {
//     num_timesteps: u32,
//     current_timestep: u32,
//     delta_time: f64
// }


// //==============================
// //         Components
// //==============================





// //==============================
// //           Systems
// //==============================


// pub fn explicit_time_integration(
//     mut query: Query<(&mut Position, &Velocity)>,
//     settings: Res<TimeIntegrationSettings>,
// ){

// }
