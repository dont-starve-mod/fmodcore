use std::io::{stdin, stdout, stderr, BufRead, BufReader, Write};
use std::error::Error;
use std::result;

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
    reader.lines().for_each(|line| {
        eprintln!("[DEBUG] handle line: {:?}", line);
        match handler.on_message_str(line.unwrap().as_str()) {
            Ok(result)=> {
                let mut out = stdout();
                writeln!(out.lock(), "[RESULT] {}",
                    json::stringify(result)).unwrap();
                out.flush().unwrap();
            },
            Err(e)=> {
                let mut err = stderr();
                writeln!(err.lock(), "[FMOD] {}",
                    e.to_string()).unwrap();
                err.flush().unwrap();
            },
        }
    });
    Ok(())
}