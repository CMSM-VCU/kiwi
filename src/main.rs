#![allow(dead_code)]
use bevy::app::AppExit;
use bevy::log::Level;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use std::fs;
use toml;

use clap::Parser;

// use kiwi::*;


/// Kiwi, an efficient peridynamics implementation
#[derive(Parser, Debug, Resource)]
#[command(author, version, about, long_about = "Path to the input file, it can be an abosolute path or it can be relative to wherever you are running the executable from.")]
struct CommandLineInputs{
    #[arg(long, short, help="Path to input file", default_value="src/testinput.toml")]
    input_file: String,
}


#[derive(Resource)]
struct KiwiConfig{
    pub table: toml::Table,
    used_keys: Vec<String>,
}

impl KiwiConfig {
    fn new(path: String) -> KiwiConfig {
        let contents:String = match fs::read_to_string(&path){
            Ok(file) => file,
            Err(error) => panic!("Could not open input file: {:?}, {:?}", error, path)
        };

        KiwiConfig{
            table: contents[..].parse::<toml::Table>().unwrap(),
            used_keys: Vec::new()
        }
    }

    fn get(&mut self, key: &str) -> Option<&toml::Value>{
        self.used_keys.push(key.to_string());
        self.table.get(key)
    }

    fn all_consumed(&self) -> bool {
        for key in self.table.keys(){
            if !self.used_keys.contains(key) {
                return false
            }
        }
        return true
    }

    fn unconsumed_keys(&self) -> Vec<&String>
    {
        let mut unconsumed_keys = Vec::new();
        for key in self.table.keys(){
            if !self.used_keys.contains(key) {
                unconsumed_keys.push(key);
            }
        }
        unconsumed_keys
    }
}


fn main() {
    let command_line_inputs = CommandLineInputs::parse();

    let input_file = KiwiConfig::new(command_line_inputs.input_file);


    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(LogPlugin{
            filter: "warn,kiwi=trace".to_string(),
            level: Level::DEBUG,
        })
        .insert_resource(input_file)
        .add_systems(Startup, read_grid)
        .add_systems(PostStartup, check_input_file_consumed)


        .add_systems(PostUpdate, exit_system)
        .run();

    // use https://crates.io/crates/kd-tree for neighbor searching
}


fn check_input_file_consumed(config: Res<KiwiConfig>){
    info!("Checking input file consumption");
    if !config.all_consumed() {
        panic!("Not all input file keys consumed, unconsumed keys:\n{:?}", config.unconsumed_keys());
    }
}

fn exit_system(mut exit: EventWriter<AppExit>) {
    debug!("Closing successfully!");
    exit.send(AppExit);
}


#[derive(Serialize, Deserialize, Default, Debug)]
struct Grid {
    pub path: String
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct Grids {
    grids: Vec<Grid>
}

impl From<Vec<Grid>> for Grids{
    fn from(value: Vec<Grid>) -> Self {
        Grids{grids: value}
    }
}


fn read_grid(mut config: ResMut<KiwiConfig>){
    info!("Reading grid files...");

    println!();
    println!();

    let g1 = Grid{path: "test1".to_string()};
    let g2 = Grid{path: "test2".to_string()};
    let gs = Grids{grids:vec![g1, g2]};
    println!("{}", toml::to_string_pretty(&gs).unwrap());

    println!();
    println!();

    for val in config.table.values(){
        dbg!(val);
    }

    println!();
    println!();
    panic!();
    let raw = config.get("Grid");
    dbg!(raw);

    if let Some(arr) = raw.unwrap().as_array(){
        for thing in arr.iter(){
            dbg!(thing);
            // thing
            // let grid: Grid = match thing.try_into(){
            //     Ok(g) => {g},
            //     Err(e) => {panic!("{:?}",e);}
            // };
            // dbg!();
        }
    }

    // if raw.unwrap().is_array(){
    //     for thing in raw.unwrap().as_array() {
    //         dbg!(thing);
    //     }
    // }

    panic!();
    // match config.get("Grid"){
    //     Some(grid) => {
    //         // grid.get("Path").expect("Grid must have a path!");
            
    //         dbg!(grid.as_str());
    //         panic!();
    //         // let grid: Grid = toml::from_str(grid.as_str().expect("IDK Why this would go wrong")).expect("Couldn't parse grid");
    //         // dbg!(grid);
    //     },
    //     None => {warn!("No grid in input file")}
    // }
    // dbg!(config.get("Grid"));

}