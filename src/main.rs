use bevy::app::AppExit;
use bevy::log::Level;
use bevy::log::LogPlugin;


use bevy::prelude::MinimalPlugins;
use kiwi::prelude::*;



fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(LogPlugin{
            filter: "warn,kiwi=trace".to_string(),
            level: Level::DEBUG,
        })
        
        .add_plugins(ParsingPlugin)
        .add_systems(Startup, create_reference_bonds_spherical)


        // exit after 1 update call
        .add_systems(PostUpdate, exit_system)

        .run();

    // use https://crates.io/crates/kd-tree for neighbor searching
}

fn exit_system(mut exit: EventWriter<AppExit>, mut iteration: Local<i32>) {
    info!("Iteration: {}",*iteration);
    if *iteration >= 0{
        info!("Closing successfully!");
        exit.send(AppExit);
    }
    *iteration += 1;
}