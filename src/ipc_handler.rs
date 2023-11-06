use crate::FmodInstance;
use json::{self, JsonValue};

pub struct IpcHandler {
    inst: FmodInstance,
}

type IpcResult<T> = Result<T, String>;

impl IpcHandler {
    pub fn new() -> IpcResult<Self> {
        match FmodInstance::new() {
            Ok(inst)=> Ok(IpcHandler { inst }),
            Err(e)=> Err(e.to_string())
        }
    }

    pub fn on_message_str(&mut self, s: &str) -> IpcResult<JsonValue> {
        let mut obj = match json::parse(s) {
            Ok(obj)=> obj,
            Err(e)=> return Err(e.to_string())
        };
        let api = obj["api"].as_str().unwrap_or("").to_string();
        let args = obj["args"].take();
        if args.is_array() {
            self.on_message(api, args)
        }
        else if args.is_null() {
            self.on_message(api, array![])
        }
        else {
            Err("Failed to get args list".to_string())
        }
    }

    pub fn on_message(&mut self, api: String, args: JsonValue) -> IpcResult<JsonValue> {
        match api.to_lowercase().as_str() {
            "playsound"=> {
                let event_name = args[0].as_str().ok_or("args[0] (event_name) must be a string".to_string())?;
                let id = args[1].as_str().ok_or("args[1] (id) must be a string".to_string())?;
                self.inst.play_sound(event_name.into(), id.into())
                    .map(|_| object! {success: true})
            },
            "killsound"=> {
                let id = args[0].as_str().ok_or("args[0] (id) must be a string".to_string())?;
                self.inst.kill_sound(&id.to_string())
                    .map(|_| object! {success: true})
            },
            "setparameter"=> {
                let id = args[0].as_str().ok_or("args[0] (id) must be a string".to_string())?;
                let param_name = args[1].as_str().ok_or("args[1] (param_name) must be a string".to_string())?;
                let value = args[2].as_f32().ok_or("args[2] (value) must be a float".to_string())?;
                self.inst.set_parameter(id.into(), param_name.into(), value)
                    .map(|_| object! {success: true})
            },
            "getinfobyid"=> {
                let id = args[0].as_str().ok_or("args[0] (id) must be a string".to_string())?;
                self.inst.get_info_by_id(&id.to_string())

            },
            "loadgameassets"=> {
                let root = args[0].as_str().ok_or("args[0] (root) must be a string".to_string())?;
                let result = self.inst.load_game_assets(root.into())?;
                std::fs::write("sfx.json", json::stringify(result)).unwrap();
                Ok(object! {})
            },
            _=> return Err(format!("Invalid api name: {}", api))
        }
    }
}

trait JsonValueExt {
    fn get_by_index(&self, index: usize) -> JsonValue;
}

// impl JsonValueExt for Vec<JsonValue> {
//     fn get_by_index(&self, index: usize) -> JsonValue {
//         match self[index] {

//         }
//     }
// }
