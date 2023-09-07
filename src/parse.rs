use bevy::math::Vec3A;
use serde::{Deserialize, Serialize};

use std::{fs, collections::HashMap};
use clap::Parser;

use crate::prelude::*;

/// Parses user input
/// Parses the command line inputs with `CommandLineInput`.
/// Reads the input file and initializes datastructures with `KiwiConfig`.
pub struct ParsingPlugin;
impl Plugin for ParsingPlugin {
    fn build(&self, app: &mut App) {

        let command_line_inputs = CommandLineInputs::parse();
        let input_file = InputFile::new(command_line_inputs.input_file.as_str());

        app
            .insert_resource(input_file)


            .add_systems(PreStartup, (
                parse_materials,
                (parse_grid,).after(parse_materials)
            ))

            .add_systems(PostStartup, check_input_file_consumed)

            ;

    }
}




/// Abstraction around a `toml::Table` that keeps track of used keys
/// Used to make sure all inputs are consumed
#[derive(Resource, Debug)]
pub struct InputFile{
    table: toml::Table,
}

impl InputFile {
    fn new(path: &str) -> InputFile {
        let contents:String = match fs::read_to_string(path){
            Ok(file) => file,
            Err(error) => panic!("Could not open input file: {error:?}, {path:?}")
        };

        InputFile{
            table: contents[..].parse::<toml::Table>().unwrap(),
        }
    }

    /// Retrieves AND CONSUMES a value from the input file
    pub fn get(&mut self, key: &str) -> Option<toml::Value>{
        self.table.remove(key)
    }
}

#[derive(Parser, Debug, Resource)]
#[command(author, version, about, long_about = "Path to the input file, it can be an abosolute path or it can be relative to wherever you are running the executable from.")]
struct CommandLineInputs{
    #[arg(long, short, help="Path to input file", default_value="test/testinput.toml")]
    input_file: String,
}





// Panics if not all keys in the input file are read
#[allow(clippy::needless_pass_by_value)]
fn check_input_file_consumed(config: Res<InputFile>){
    info!("Checking input file consumption...");
    assert!(config.table.is_empty(), "Not all input file keys consumed, unconsumed keys:\n{:?}", config.table.keys().collect::<Vec<&String>>());
    info!("All inputs used!");
}

#[derive(Deserialize, Serialize, Resource)]
struct MaterialsList(Vec<BondBasedLinearElastic>);

fn parse_materials(
    mut config: ResMut<InputFile>,
    mut commands: Commands
){
    
    let mats = config.get("Material").expect("No `[[Material]]`s defined in the input file");
    
    let list = MaterialsList::deserialize(mats).expect("Could not deserialize materials");
    let map = Materials(list.0.into_iter().map(|x| {(x.id, x)}).collect::<HashMap<u32, BondBasedLinearElastic>>());
    commands.insert_resource(map);
}

/// Consumes grid field of `InputFile` and creates material points from the grid file (csv format)
/// Possible data includes:
///     `Position` specified by the x, y, and z column
///     `Displacement` specified by the ux, uy, and uz column
///     `MaterialID` (required) specified by the mat column
///     `Mass` specified by the mass column
fn parse_grid(
    mut config: ResMut<InputFile>,
    mut commands: Commands
){
    let grids = config.get("Grid").unwrap();
    let grids = grids.as_array().expect("msg");

    for grid in grids{
        // Grid must be a table that contains a path key
        let path = grid
            .as_table().expect("[[Grid]] must be a toml table") // Gets table
            .get("path").expect("[[Grid]] must have 'path' key!") // Gets path
            .as_str().expect("the path key in a [[Grid]] must be a string"); // Assets path is a string

        // CSV Properties
        let mut rdr = match csv::ReaderBuilder::new()
            .has_headers(true)
            .delimiter(b',')
            .comment(Some(b'#'))
            .from_path(path){
                Ok(reader) => reader,
                Err(err) => panic!("Could not read grid file at {path}. Error: {err}"),
            };
        assert!(rdr.has_headers());

        
        let trimmed_headers: csv::StringRecord = rdr.headers().unwrap().iter().map(|x|{x.trim().to_lowercase()}).collect();
        rdr.set_headers(trimmed_headers);
        

        // Parse record and create `MaterialPoints` from grid file
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
                pos.x = str::parse::<f32>(record.remove("x").unwrap().as_str()).expect("Could not parse point position x");
            }
            if record.contains_key("y"){
                pos.y = str::parse::<f32>(record.remove("y").unwrap().as_str()).expect("Could not parse point position y");
            }
            if record.contains_key("z"){
                pos.z = str::parse::<f32>(record.remove("z").unwrap().as_str()).expect("Could not parse point position z");
            }

            let mut disp: Vec3A = Vec3A::ZERO;
            if record.contains_key("ux"){
                disp.x = str::parse::<f32>(record.remove("ux").unwrap().as_str()).expect("Could not parse ux");
            }
            if record.contains_key("uy"){
                disp.y = str::parse::<f32>(record.remove("uy").unwrap().as_str()).expect("Could not parse uy");
            }
            if record.contains_key("uz"){
                disp.z = str::parse::<f32>(record.remove("uz").unwrap().as_str()).expect("Could not parse uz");
            }

            let mut vel: Vec3A = Vec3A::ZERO;
            if record.contains_key("vx"){
                vel.x = str::parse::<f32>(record.remove("vx").unwrap().as_str()).expect("Could not parse vx");
            }
            if record.contains_key("vy"){
                vel.y = str::parse::<f32>(record.remove("vy").unwrap().as_str()).expect("Could not parse vy");
            }
            if record.contains_key("vz"){
                vel.z = str::parse::<f32>(record.remove("vz").unwrap().as_str()).expect("Could not parse vz");
            }

            let mut force: Vec3A = Vec3A::ZERO;
            if record.contains_key("fx"){
                force.x = str::parse::<f32>(record.remove("fx").unwrap().as_str()).expect("Could not parse fx");
            }
            if record.contains_key("fy"){
                force.y = str::parse::<f32>(record.remove("fy").unwrap().as_str()).expect("Could not parse fy");
            }
            if record.contains_key("fz"){
                force.z = str::parse::<f32>(record.remove("fz").unwrap().as_str()).expect("Could not parse fz");
            }

            // TODO: Mass can be handeled multiple ways, but needs to be explicitly defined
            // Options:
            //      Explicitly stated in grid file
            //      Volume in grid file, density in from material
            //      Both, but need to be checked for consistency
            let mut mass: f32 = 0.0;
            if record.contains_key("mass"){
                mass = str::parse::<f32>(record.remove("mass").unwrap().as_str()).expect("Could not parse mass");
            }


            let mat: u32 = if record.contains_key("mat"){
                str::parse::<u32>(record.remove("mat").unwrap().as_str()).expect("Could not parse material, must be positive unsigned 32 bit integer")
            }
            else{
                panic!("Could not find column 'mat' in grid file. Each points must have a material number");
            };


            // Make sure there are no unconsumed keys (this will find typos in the header)
            assert!(record.keys().len() == 0, "Unused header in grid file: {:?}", record.keys());

            trace!("Adding point: {:?}", pos);
            // Spawn `MaterialPoint` in ECS world
            commands.spawn((
                MaterialPoint,
                Position(pos),
                Displacement(disp),
                Velocity(vel),
                Force(force),
                Mass(mass),
                MaterialID(mat)
            ));
        }
    }
}
