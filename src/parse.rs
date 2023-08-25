use bevy::prelude::*;
use bevy::math::Vec3A;

use std::{fs, collections::HashMap};
use clap::Parser;

use crate::node;


pub struct ParsingPlugin;
impl Plugin for ParsingPlugin {
    fn build(&self, app: &mut App) {

        let command_line_inputs = CommandLineInputs::parse();
        let input_file = KiwiConfig::new(command_line_inputs.input_file);

        app
            .insert_resource(input_file)


            // Throw error if e
            .add_systems(Startup, parse_grid)
            .add_systems(PostStartup, check_input_file_consumed)

            ;

    }
}




/// Abstraction around a toml::Table that keeps track of used keys
/// Used to make sure all inputs are consumed
#[derive(Resource)]
pub struct KiwiConfig{
    table: toml::Table,
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

    fn unconsumed_keys(&self) -> Vec<&String> {
        let mut unconsumed_keys = Vec::new();
        for key in self.table.keys(){
            if !self.used_keys.contains(key) {
                unconsumed_keys.push(key);
            }
        }
        unconsumed_keys
    }
}

#[derive(Parser, Debug, Resource)]
#[command(author, version, about, long_about = "Path to the input file, it can be an abosolute path or it can be relative to wherever you are running the executable from.")]
struct CommandLineInputs{
    #[arg(long, short, help="Path to input file", default_value="src/testinput.toml")]
    input_file: String,
}


// Panics if not all keys in the input file are read
fn check_input_file_consumed(config: Res<KiwiConfig>){
    info!("Checking input file consumption");
    if !config.all_consumed() {
        panic!("Not all input file keys consumed, unconsumed keys:\n{:?}", config.unconsumed_keys());
    }
}



fn parse_grid(
    mut config: ResMut<KiwiConfig>,
    mut commands: Commands
){
    let grids = config.get("Grid")
        .expect("No [[Grid]]s in input file")
        .as_array().expect("Grid not in an array, use [[Grid]] tag to specify that it's an array of tables https://toml.io/en/v1.0.0#array-of-tables");

    for grid in grids{
        // Grid must be a table that contains a path key
        let path = grid
            .as_table().expect("[[Grid]] must be a toml table") // Gets table
            .get("path").expect("[[Grid]] must have 'path' key!") // Gets path
            .as_str().expect("the path key in a [[Grid]] must be a string"); // Assets path is a string

        let mut rdr = match csv::ReaderBuilder::new()
            .has_headers(true)
            .delimiter(b',')
            .comment(Some(b'#'))
            .from_path(path){
                Ok(reader) => reader,
                Err(err) => panic!("Could not read grid file at {path}. Error: {err}"),
            };
        
        dbg!(rdr.headers().expect("Grid CSV must have headers"));

        // Parse record and create nodes from grid file
        for result in rdr.deserialize(){

            // Converts each entry (row) in the reader to a hashmap where the column header is the key
            // use record.remove(key) to get the value because there is a check to see if 
            // all entries are consumed to prevent typos in headers from silently not working
            let mut record: HashMap<String, String> = match result{
                Ok(record) => record,
                Err(err) => panic!("Could not parse record: {err}"),
            };

            let mut pos: Vec3A = Vec3A::ZERO;
            if record.contains_key("x"){
                pos.x = str::parse::<f32>(record.remove("x").unwrap().as_str()).expect("Could not parse float, may need spaces in header");
            }
            if record.contains_key("y"){
                pos.y = str::parse::<f32>(record.remove("y").unwrap().as_str()).expect("Could not parse float, may need spaces in header");
            }
            if record.contains_key("z"){
                pos.z = str::parse::<f32>(record.remove("z").unwrap().as_str()).expect("Could not parse float, may need spaces in header");
            }

            let mut disp: Vec3A = Vec3A::ZERO;
            if record.contains_key("ux"){
                disp.x = str::parse::<f32>(record.remove("ux").unwrap().as_str()).expect("Could not parse float, may need spaces in header");
            }
            if record.contains_key("uy"){
                disp.y = str::parse::<f32>(record.remove("uy").unwrap().as_str()).expect("Could not parse float, may need spaces in header");
            }
            if record.contains_key("uz"){
                disp.z = str::parse::<f32>(record.remove("uz").unwrap().as_str()).expect("Could not parse float, may need spaces in header");
            }

            let mut vel: Vec3A = Vec3A::ZERO;
            if record.contains_key("vx"){
                vel.x = str::parse::<f32>(record.remove("vx").unwrap().as_str()).expect("Could not parse float, may need spaces in header");
            }
            if record.contains_key("vy"){
                vel.y = str::parse::<f32>(record.remove("vy").unwrap().as_str()).expect("Could not parse float, may need spaces in header");
            }
            if record.contains_key("vz"){
                vel.z = str::parse::<f32>(record.remove("vz").unwrap().as_str()).expect("Could not parse float, may need spaces in header");
            }

            // Panic if there are unconsumed keys
            if record.keys().len() != 0{
                panic!("Unused header in grid file: {:?}", record.keys());
            }

            // Spawn node in ECS world
            commands.spawn((
                node::Node,
                node::Position(pos),
                node::Displacement(disp),
                node::Velocity(vel)
            ));
        }
    }
}
