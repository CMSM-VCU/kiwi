use bevy::prelude::*;
use toml;

use std::fs;
use clap::Parser;

// Parses input file
pub struct ParsingPlugin;
impl Plugin for ParsingPlugin {
    fn build(&self, app: &mut App) {

        let command_line_inputs = CommandLineInputs::parse();
        let input_file = KiwiConfig::new(command_line_inputs.input_file);

        app
            .insert_resource(input_file)


            // Throw error if e
            .add_systems(PostStartup, check_input_file_consumed);
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
