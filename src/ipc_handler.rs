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

    pub fn on_message(&mut self, api: String, mut args: JsonValue) -> IpcResult<JsonValue> {
        let result = match api.as_str() {
            "PlaySound"=> {
                let event_name = args[0].as_str().ok_or("args[0] (event_name) must be a string".to_string())?;
                let id = args[1].as_str().ok_or("args[1] (id) must be a string".to_string())?;
                self.inst.play_sound(event_name.into(), id.into(), None)
                    .map(|_| object! {success: true})
            },
            "KillSound"=> {
                let id = args[0].as_str().ok_or("args[0] (id) must be a string".to_string())?;
                self.inst.kill_sound(&id.to_string())
                    .map(|_| object! {success: true})
            },
            "SetParameter"=> {
                let id = args[0].as_str().ok_or("args[0] (id) must be a string".to_string())?;
                let param_name = args[1].as_str().ok_or("args[1] (param_name) must be a string".to_string())?;
                let value = args[2].as_f32().ok_or("args[2] (value) must be a float".to_string())?;
                self.inst.set_parameter(id.into(), param_name.into(), value)
                    .map(|_| object! {success: true})
            },
            "PlaySoundWithParams"=> {
                let event_name = args[0].as_str().ok_or("args[0] (event_name) must be a string".to_string())?;
                let id = args[1].as_str().ok_or("args[1] (id) must be a string".to_string())?;
                if !args[2].is_object(){
                    return Err("args[2] (params) must be a object {string: float}".to_string());
                }
                self.inst.play_sound(event_name.into(), id.into(), Some(args[2].take()))
                    .map(|_| object! {success: true})
            }
            "GetInfoById"=> {
                let id = args[0].as_str().ok_or("args[0] (id) must be a string".to_string())?;
                self.inst.get_info_by_id(&id.to_string())
            },
            "GetAllInfo"=> {
                self.inst.get_all_playing_info()
                    .map(|result|result.into())
            }
            "SetVolume"=> {
                let volume = args[0].as_f32().ok_or("args[0] (volume) must be a string".to_string())?;
                self.inst.set_global_volume(volume)
                    .map(|_| object! {success: true})
            },
            "LoadGameAssets"=> {
                let root = args[0].as_str().ok_or("args[0] (root) must be a string".to_string())?;
                self.inst.unload_all()?; // TODO: only unload game assets?
                let result = self.inst.load_game_assets(root.into())?;
                Ok(result)
            },
            _=> return Err(format!("Invalid api name: {}", api))
        }?;
        Ok(object! {"api": api, "result": result})
    }
}