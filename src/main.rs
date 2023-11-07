use std::io::{stdin, BufRead, BufReader};
use std::error::Error;

#[macro_use]
extern crate json;

mod fmod;
mod ipc_handler;
use fmodcore::*;

/*  test input
{"api": "LoadGameAssets", "args": ["/Users/wzh/Library/Application Support/Steam/steamapps/common/Don't Starve Together/dontstarve_steam.app/Contents/data/sound"]}
{"api": "PlaySound", "args": ["turnoftides/common/together/moon_glass/mine", "DEBUG"]}
{"api": "KillSound", "args": ["DEBUG"]}
{"api": "PlaySound", "args": ["turnoftides/common/together/water/wave/LP", "WAVE"]}
{"api": "KillSound", "args": ["WAVE"]}
{"api": "SetVolume", "args": [0.1]}
{"api": "GetAllInfo"}
*/

fn main() -> Result<(), Box<dyn Error>> {
    let mut handler = ipc_handler::IpcHandler::new().unwrap_or_else(|e|{
        eprintln!("Failed to create handler: {}", e.to_string());
        std::process::exit(101);
    });
    
    let reader = BufReader::new(stdin());
    reader.lines().for_each(|s| {
        match handler.on_message_str(s.unwrap().as_str()) {
            Ok(result)=> println!("[RESULT] {}", json::stringify(result)),
            Err(e)=> eprintln!("[FMOD] {}", e),
        }
    });
    Ok(())
}