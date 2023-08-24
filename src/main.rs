#![allow(dead_code)]
use bevy::app::AppExit;
use bevy::log::Level;
use bevy::log::LogPlugin;
use bevy::prelude::*;

use parse::ParsingPlugin;




// My libs
mod parse;
// pub mod prelude{
//     use parse::*;
// }


/// Kiwi, an efficient peridynamics implementation


fn main() {



    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(LogPlugin{
            filter: "warn,kiwi=trace".to_string(),
            level: Level::DEBUG,
        })
        
        .add_plugins(parse::ParsingPlugin)
        // .add_systems(Startup, parse_grid)


        // exit after 1 update call
        .add_systems(PostUpdate, exit_system)
        .run();

    // use https://crates.io/crates/kd-tree for neighbor searching
}


fn exit_system(mut exit: EventWriter<AppExit>) {
    debug!("Closing successfully!");
    exit.send(AppExit);
}


// #[derive(Serialize, Deserialize, Default, Debug)]
// struct Grid {
//     pub path: String
// }

// #[derive(Serialize, Deserialize, Default, Debug)]
// struct Grids {
//     grids: Vec<Grid>
// }

// impl From<Vec<Grid>> for Grids{
//     fn from(value: Vec<Grid>) -> Self {
//         Grids{grids: value}
//     }
// }


// fn parse_grid(mut config: ResMut<KiwiConfig>){
//     info!("Reading grid files...");

//     println!();
//     println!();

//     let g1 = Grid{path: "test1".to_string()};
//     let g2 = Grid{path: "test2".to_string()};
//     let gs = Grids{grids:vec![g1, g2]};
//     println!("{}", toml::to_string_pretty(&gs).unwrap());

//     println!();
//     println!();

//     for val in config.table.values(){
//         dbg!(val);
//     }

//     println!();
//     println!();
//     panic!();
//     let raw = config.get("Grid");
//     dbg!(raw);

//     if let Some(arr) = raw.unwrap().as_array(){
//         for thing in arr.iter(){
//             dbg!(thing);
//             // thing
//             // let grid: Grid = match thing.try_into(){
//             //     Ok(g) => {g},
//             //     Err(e) => {panic!("{:?}",e);}
//             // };
//             // dbg!();
//         }
//     }

//     // if raw.unwrap().is_array(){
//     //     for thing in raw.unwrap().as_array() {
//     //         dbg!(thing);
//     //     }
//     // }

//     panic!();
//     // match config.get("Grid"){
//     //     Some(grid) => {
//     //         // grid.get("Path").expect("Grid must have a path!");
            
//     //         dbg!(grid.as_str());
//     //         panic!();
//     //         // let grid: Grid = toml::from_str(grid.as_str().expect("IDK Why this would go wrong")).expect("Couldn't parse grid");
//     //         // dbg!(grid);
//     //     },
//     //     None => {warn!("No grid in input file")}
//     // }
//     // dbg!(config.get("Grid"));

// }