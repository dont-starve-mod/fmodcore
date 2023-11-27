use std::collections::HashMap;

mod fmod;
#[macro_use]
extern crate json;
use json::JsonValue;
use std::fs;
use std::ffi::{CStr, CString, OsString, c_void};
use std::path::{PathBuf, MAIN_SEPARATOR};
use std::ptr::null_mut;

pub trait ToFmodResultTrait {
    fn print(&self);
    fn as_result(&self) -> FmodResult<()>;
}

/// add util fns for fmod result code
impl ToFmodResultTrait for fmod::FMOD_RESULT {
    #[inline]
    fn print(&self) {
        println!("{}", format_fmod_result(*self));
    }

    #[inline]
    fn as_result(&self) -> FmodResult<()> {
        match *self {
            0=> Ok(()),
            n=> Err(format_fmod_result(n)),
        }
    }
}

/// convert fmod error code to human string
fn format_fmod_result(result: fmod::FMOD_RESULT)-> String {
    match result {  
        0 => "FMOD_RESULT_FMOD_OK", 
        1 => "FMOD_RESULT_FMOD_ERR_ALREADYLOCKED",
        2 => "FMOD_RESULT_FMOD_ERR_BADCOMMAND",
        3 => "FMOD_RESULT_FMOD_ERR_CDDA_DRIVERS",
        4 => "FMOD_RESULT_FMOD_ERR_CDDA_INIT",
        5 => "FMOD_RESULT_FMOD_ERR_CDDA_INVALID_DEVICE",
        6 => "FMOD_RESULT_FMOD_ERR_CDDA_NOAUDIO",
        7 => "FMOD_RESULT_FMOD_ERR_CDDA_NODEVICES",
        8 => "FMOD_RESULT_FMOD_ERR_CDDA_NODISC",
        9 => "FMOD_RESULT_FMOD_ERR_CDDA_READ",
        10 => "FMOD_RESULT_FMOD_ERR_CHANNEL_ALLOC",
        11 => "FMOD_RESULT_FMOD_ERR_CHANNEL_STOLEN",
        12 => "FMOD_RESULT_FMOD_ERR_COM",
        13 => "FMOD_RESULT_FMOD_ERR_DMA",
        14 => "FMOD_RESULT_FMOD_ERR_DSP_CONNECTION",
        15 => "FMOD_RESULT_FMOD_ERR_DSP_FORMAT",
        16 => "FMOD_RESULT_FMOD_ERR_DSP_NOTFOUND",
        17 => "FMOD_RESULT_FMOD_ERR_DSP_RUNNING",
        18 => "FMOD_RESULT_FMOD_ERR_DSP_TOOMANYCONNECTIONS",
        19 => "FMOD_RESULT_FMOD_ERR_FILE_BAD",
        20 => "FMOD_RESULT_FMOD_ERR_FILE_COULDNOTSEEK",
        21 => "FMOD_RESULT_FMOD_ERR_FILE_DISKEJECTED",
        22 => "FMOD_RESULT_FMOD_ERR_FILE_EOF",
        23 => "FMOD_RESULT_FMOD_ERR_FILE_NOTFOUND",
        24 => "FMOD_RESULT_FMOD_ERR_FILE_UNWANTED",
        25 => "FMOD_RESULT_FMOD_ERR_FORMAT",
        26 => "FMOD_RESULT_FMOD_ERR_HTTP",
        27 => "FMOD_RESULT_FMOD_ERR_HTTP_ACCESS",
        28 => "FMOD_RESULT_FMOD_ERR_HTTP_PROXY_AUTH",
        29 => "FMOD_RESULT_FMOD_ERR_HTTP_SERVER_ERROR",
        30 => "FMOD_RESULT_FMOD_ERR_HTTP_TIMEOUT",
        31 => "FMOD_RESULT_FMOD_ERR_INITIALIZATION",
        32 => "FMOD_RESULT_FMOD_ERR_INITIALIZED",
        33 => "FMOD_RESULT_FMOD_ERR_INTERNAL",
        34 => "FMOD_RESULT_FMOD_ERR_INVALID_ADDRESS",
        35 => "FMOD_RESULT_FMOD_ERR_INVALID_FLOAT",
        36 => "FMOD_RESULT_FMOD_ERR_INVALID_HANDLE",
        37 => "FMOD_RESULT_FMOD_ERR_INVALID_PARAM",
        38 => "FMOD_RESULT_FMOD_ERR_INVALID_POSITION",
        39 => "FMOD_RESULT_FMOD_ERR_INVALID_SPEAKER",
        40 => "FMOD_RESULT_FMOD_ERR_INVALID_SYNCPOINT",
        41 => "FMOD_RESULT_FMOD_ERR_INVALID_VECTOR",
        42 => "FMOD_RESULT_FMOD_ERR_MAXAUDIBLE",
        43 => "FMOD_RESULT_FMOD_ERR_MEMORY",
        44 => "FMOD_RESULT_FMOD_ERR_MEMORY_CANTPOINT",
        45 => "FMOD_RESULT_FMOD_ERR_MEMORY_SRAM",
        46 => "FMOD_RESULT_FMOD_ERR_NEEDS2D",
        47 => "FMOD_RESULT_FMOD_ERR_NEEDS3D",
        48 => "FMOD_RESULT_FMOD_ERR_NEEDSHARDWARE",
        49 => "FMOD_RESULT_FMOD_ERR_NEEDSSOFTWARE",
        50 => "FMOD_RESULT_FMOD_ERR_NET_CONNECT",
        51 => "FMOD_RESULT_FMOD_ERR_NET_SOCKET_ERROR",
        52 => "FMOD_RESULT_FMOD_ERR_NET_URL",
        53 => "FMOD_RESULT_FMOD_ERR_NET_WOULD_BLOCK",
        54 => "FMOD_RESULT_FMOD_ERR_NOTREADY",
        55 => "FMOD_RESULT_FMOD_ERR_OUTPUT_ALLOCATED",
        56 => "FMOD_RESULT_FMOD_ERR_OUTPUT_CREATEBUFFER",
        57 => "FMOD_RESULT_FMOD_ERR_OUTPUT_DRIVERCALL",
        58 => "FMOD_RESULT_FMOD_ERR_OUTPUT_ENUMERATION",
        59 => "FMOD_RESULT_FMOD_ERR_OUTPUT_FORMAT",
        60 => "FMOD_RESULT_FMOD_ERR_OUTPUT_INIT",
        61 => "FMOD_RESULT_FMOD_ERR_OUTPUT_NOHARDWARE",
        62 => "FMOD_RESULT_FMOD_ERR_OUTPUT_NOSOFTWARE",
        63 => "FMOD_RESULT_FMOD_ERR_PAN",
        64 => "FMOD_RESULT_FMOD_ERR_PLUGIN",
        65 => "FMOD_RESULT_FMOD_ERR_PLUGIN_INSTANCES",
        66 => "FMOD_RESULT_FMOD_ERR_PLUGIN_MISSING",
        67 => "FMOD_RESULT_FMOD_ERR_PLUGIN_RESOURCE",
        68 => "FMOD_RESULT_FMOD_ERR_PRELOADED",
        69 => "FMOD_RESULT_FMOD_ERR_PROGRAMMERSOUND",
        70 => "FMOD_RESULT_FMOD_ERR_RECORD",
        71 => "FMOD_RESULT_FMOD_ERR_REVERB_INSTANCE",
        72 => "FMOD_RESULT_FMOD_ERR_SUBSOUND_ALLOCATED",
        73 => "FMOD_RESULT_FMOD_ERR_SUBSOUND_CANTMOVE",
        74 => "FMOD_RESULT_FMOD_ERR_SUBSOUND_MODE",
        75 => "FMOD_RESULT_FMOD_ERR_SUBSOUNDS",
        76 => "FMOD_RESULT_FMOD_ERR_TAGNOTFOUND",
        77 => "FMOD_RESULT_FMOD_ERR_TOOMANYCHANNELS",
        78 => "FMOD_RESULT_FMOD_ERR_UNIMPLEMENTED",
        79 => "FMOD_RESULT_FMOD_ERR_UNINITIALIZED",
        80 => "FMOD_RESULT_FMOD_ERR_UNSUPPORTED",
        81 => "FMOD_RESULT_FMOD_ERR_UPDATE",
        82 => "FMOD_RESULT_FMOD_ERR_VERSION",
        83 => "FMOD_RESULT_FMOD_ERR_EVENT_FAILED",
        84 => "FMOD_RESULT_FMOD_ERR_EVENT_INFOONLY",
        85 => "FMOD_RESULT_FMOD_ERR_EVENT_INTERNAL",
        86 => "FMOD_RESULT_FMOD_ERR_EVENT_MAXSTREAMS",
        87 => "FMOD_RESULT_FMOD_ERR_EVENT_MISMATCH",
        88 => "FMOD_RESULT_FMOD_ERR_EVENT_NAMECONFLICT",
        89 => "FMOD_RESULT_FMOD_ERR_EVENT_NOTFOUND",
        90 => "FMOD_RESULT_FMOD_ERR_EVENT_NEEDSSIMPLE",
        91 => "FMOD_RESULT_FMOD_ERR_EVENT_GUIDCONFLICT",
        92 => "FMOD_RESULT_FMOD_ERR_EVENT_ALREADY_LOADED",
        93 => "FMOD_RESULT_FMOD_ERR_MUSIC_UNINITIALIZED",
        94 => "FMOD_RESULT_FMOD_ERR_MUSIC_NOTFOUND",
        95 => "FMOD_RESULT_FMOD_ERR_MUSIC_NOCALLBACK",
        n => panic!("Fmod emitted unknown error code: {}", n),
    }.to_string()
}

type FmodResult<T> = Result<T, String>;

pub struct FmodInstance {
    system: *mut fmod::FMOD_EVENTSYSTEM,
    projects: Vec<*mut fmod::FMOD_EVENTPROJECT>,
    playing_events: HashMap<String, *mut fmod::FMOD_EVENT>,
    volume: f32,
    major_categories: HashMap<String, *mut fmod::FMOD_EVENTCATEGORY>,
}

#[derive(Debug, Clone)]
pub struct FmodParam {
    name: String,
    range: (f32, f32),
    current: Option<f32>,
    seekspeed: Option<f32>,
}

impl From<FmodParam> for JsonValue {
    fn from(value: FmodParam) -> Self {
        if value.current.is_some() {
            object! {
                name: value.name,
                range: [value.range.0, value.range.1],
                current: value.current.unwrap(),
                seekspeed: value.seekspeed,
            }
        }
        else {
            object! {
                name: value.name,
                range: [value.range.0, value.range.1],
                seekspeed: value.seekspeed,
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct FmodEventInfo {
    name: String,
    group: String,
    project: String,
    category: String,
    lengthms: i32,
    param_list: Vec<FmodParam>,
}

impl FmodEventInfo {
    #[inline]
    pub fn get_path(&self) -> String {
        format!("{}/{}/{}", self.project, self.group, self.name)
    }

    pub fn get_hash(&self) -> u32 {
        self.get_path().as_bytes().iter()
            .fold::<u64, _>(0, |hash, x|{
                let x = match *x as u64 {
                    n @ 65..=90 => n + 32,
                    n if n > 127 => n + 0xFFFFFF00,
                    n => n
                };
                (x + (hash << 6) + (hash << 16) - hash) & 0xFFFFFFFF
            })
        as u32
    }
}

impl From<FmodEventInfo> for JsonValue {
    fn from(value: FmodEventInfo) -> Self {
        object! {
            hash: value.get_hash(),
            path: value.get_path(),
            name: value.name,
            group: value.group,
            project: value.project,
            category: value.category,
            lengthms: value.lengthms,
            param_list: value.param_list,
        }
    }
}

pub struct FmodPlayingEventInfo {
    playing: bool,
    param_list: Vec<FmodParam>,
    positionms: i32,
    lengthms: i32,
}

impl From<FmodPlayingEventInfo> for JsonValue {
    fn from(value: FmodPlayingEventInfo) -> Self {
        object! {
            playing: value.playing,
            param_list: value.param_list,
            positionms: value.positionms,
            lengthms: value.lengthms,
        }
    }
}

#[inline]
fn convert_mem_to_string(mem: *const i8) -> String {
    unsafe { CStr::from_ptr(mem) }
        .to_string_lossy()
        .to_string()
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
impl FmodInstance {
    pub fn new() -> FmodResult<Self> {
        println!("[FMOD] create new event system");
        let mut system = null_mut();
        unsafe {
            fmod::FMOD_EventSystem_Create(&mut system).as_result()?;
        };
        println!("[FMOD] init event system");
        unsafe {
            fmod::FMOD_EventSystem_Init(system, 64, 0, null_mut(), 0).as_result()?;
        };

        if system.is_null() {
            Err("[FMOD] failed to construct FmodInstance: system ptr is null".into())
        }
        else {
            Ok(FmodInstance {
                system,
                projects: Vec::new(),
                playing_events: HashMap::new(),
                volume: 1.0,
                major_categories: HashMap::new(),
            })
        }
    }

    /// load all *.fev asset files in game data directory.
    /// get information of all fmod events in loaded projects.
    pub fn load_game_assets(&mut self, mut root: String) -> FmodResult<JsonValue> {
        let mut result = object! {};
        if !root.ends_with(MAIN_SEPARATOR) {
            root.push(MAIN_SEPARATOR);
        }
        let rootpath = PathBuf::from(&root);
        if rootpath.is_dir() {
            unsafe { fmod::FMOD_EventSystem_SetMediaPath(self.system, root.as_mut_ptr().cast()); }
        }
        else {
            return Err(format!("game root is not a dir: {}", root));
        }
            
        for entry in fs::read_dir(rootpath).map_err(|e| format!("Failed to read dir: {}\n{}", root, e))?.flatten() {
            let path = entry.path();
            if !(path.is_file() && path.extension() == Some(&OsString::from("fev"))){
                continue;
            }
            let name = path.file_name().unwrap();
            print!("[FMOD] 🎵 - {:?} ", name);
            let mut project = null_mut();
            let name_string = match name.to_str() {
                Some(s)=> s.to_string(),
                None=> {
                    eprintln!("Failed to parse OsString");
                    continue;
                }
            };
            let name_cstring = match CString::new(name_string.as_bytes().to_vec()) {
                Ok(s)=> s,
                Err(e)=> {
                    eprintln!("Failed to construct CString: {}", e);
                    continue;
                }
            };

            unsafe {
                match fmod::FMOD_EventSystem_Load(
                    self.system, name_cstring.as_ptr(), 
                    null_mut(),
                    &mut project
                ).as_result() {
                    Ok(_)=> println!("...OK"),
                    Err(e)=> println!("...{}", e)
                }
            }
            if project.is_null() {
                continue;
            }
            
            self.projects.push(project);
            // iter all events in project
            let mut event_map = HashMap::new();
            let mut project_info = fmod::FMOD_EVENT_PROJECTINFO::default();
            unsafe { fmod::FMOD_EventProject_GetInfo(project, &mut project_info).as_result()?; }
            let name = convert_mem_to_string(project_info.name.as_ptr());
            let mut num_events = 0;
            unsafe { fmod::FMOD_EventProject_GetNumEvents(project, &mut num_events).as_result()?; }
            for i in 0..num_events {
                let mut event = null_mut();
                unsafe { fmod::FMOD_EventProject_GetEventByProjectID(project, i as u32, 4, &mut event).as_result()?; }
                if !event.is_null() {
                    if let Ok(info) = self.get_event_info(event) {
                        event_map.insert(
                            info.get_path(),
                            info,
                        );
                    };
                }
            }

            result[name] = object! {
                filename: path.file_name().unwrap().to_string_lossy().to_string(),
                fullpath: path.to_string_lossy().to_string(),
                event_map: event_map,
            }
            
        }

        // list all categories
        let mut num_categories = 0;
        let mut category = null_mut();
        let mut name = null_mut();
        unsafe { fmod::FMOD_EventSystem_GetNumCategories(self.system, &mut num_categories).as_result()?; }
        for i in 0..num_categories {
            unsafe {
                fmod::FMOD_EventSystem_GetCategoryByIndex(self.system, i, &mut category).as_result()?;
                fmod::FMOD_EventCategory_GetInfo(category, null_mut(), &mut name).as_result()?;
                eprintln!("[DEBUG] cate name: {}", convert_mem_to_string(name));
            }
            self.major_categories.insert(convert_mem_to_string(name), category);
        }
        
        Ok(result)
    }

    pub fn unload_all(&mut self) -> FmodResult<()> {
        unsafe { fmod::FMOD_EventSystem_Unload(self.system).as_result()?; }
        Ok(())
    }

    /// get information of a fmod event
    pub fn get_event_info(&self, event: *mut fmod::FMOD_EVENT) -> FmodResult<FmodEventInfo> {
        // name
        let mut name = null_mut();
        let mut info = fmod::FMOD_EVENT_INFO::default();
        unsafe {
            fmod::FMOD_Event_GetInfo(event, std::ptr::null_mut(), &mut name, &mut info).as_result()?;
        }
        // category
        let mut category = null_mut();
        let mut category_name = null_mut();
        let mut category_name_list = Vec::<String>::new();
        unsafe {
            fmod::FMOD_Event_GetCategory(event, &mut category).as_result()?;
        }
        let category_path = loop {
            unsafe {
                fmod::FMOD_EventCategory_GetInfo(category, null_mut(), &mut category_name).as_result()?;
            };
            category_name_list.push(convert_mem_to_string(category_name));
            let mut parent = null_mut();
            unsafe {
                fmod::FMOD_EventCategory_GetParentCategory(category, &mut parent).as_result()?;
            };
            if parent.is_null() {
                category_name_list.reverse();
                break category_name_list.join("/")
            }
            else {
                category = parent;
            }
        };
        // group
        let mut group = null_mut();
        unsafe {
            fmod::FMOD_Event_GetParentGroup(event, &mut group).as_result()?;
        }
        let mut group_name = null_mut();
        let mut group_name_list = Vec::<String>::new();
        let group_path = loop {
            unsafe {
                fmod::FMOD_EventGroup_GetInfo(group, null_mut(), &mut group_name).as_result()?;
            }
            group_name_list.push(convert_mem_to_string(group_name));
            let mut parent = null_mut();
            unsafe {
                fmod::FMOD_EventGroup_GetParentGroup(group, &mut parent).as_result()?;
            }
            if parent.is_null() {
                group_name_list.reverse();
                break group_name_list.join("/")
            }
            else {
                group = parent;
            }
        };
        // project
        let mut project = null_mut();
        unsafe {
            fmod::FMOD_EventGroup_GetParentProject(group, &mut project).as_result()?;
        };
        let mut project_info = fmod::FMOD_EVENT_PROJECTINFO::default();
        unsafe {
            fmod::FMOD_EventProject_GetInfo(project, &mut project_info).as_result()?;
        };
        // parameter
        let mut num_params = 0;
        unsafe {
            fmod::FMOD_Event_GetNumParameters(event, &mut num_params).as_result()?;
        }
        let mut param_list = Vec::new();
        for i in 0..num_params {
            let mut param = null_mut();
            let mut param_name = null_mut();
            unsafe {
                fmod::FMOD_Event_GetParameterByIndex(event, i, &mut param).as_result()?;
                fmod::FMOD_EventParameter_GetInfo(param, null_mut(), &mut param_name).as_result()?;
            }  
            if !param_name.is_null() {
                let (mut min, mut max, mut seekspeed) = (0.0, 0.0, 0.0);
                unsafe {
                    fmod::FMOD_EventParameter_GetRange(param, &mut min, &mut max).as_result()?;
                    fmod::FMOD_EventParameter_GetSeekSpeed(param, &mut seekspeed).as_result()?;
                }
                param_list.push(FmodParam{
                    name: convert_mem_to_string(param_name),
                    range: (min, max),
                    seekspeed: Some(seekspeed),
                    current: None, // get current parameter value by `get_playing_event_info()`
                });
            }
        }
        Ok(FmodEventInfo{
            name: convert_mem_to_string(name),
            group: group_path,
            category: category_path,
            project: convert_mem_to_string(project_info.name.as_ptr()),
            lengthms: info.lengthms,
            param_list,
        })
    }

    /// get playing info of event, including time/parameter
    pub fn get_playing_event_info(&self, event: *mut fmod::FMOD_EVENT) -> FmodResult<FmodPlayingEventInfo> {
        let mut num_params = 0;
        unsafe { fmod::FMOD_Event_GetNumParameters(event, &mut num_params).as_result()? }
        let mut param_list = Vec::new();
        for i in 0..num_params {
            let mut param = null_mut();
            let mut param_name = null_mut();
            unsafe {
                fmod::FMOD_Event_GetParameterByIndex(event, i, &mut param).as_result()?;
                fmod::FMOD_EventParameter_GetInfo(param, null_mut(), &mut param_name).as_result()?;
            }
            let (mut min, mut max, mut current) = (0.0, 0.0, 0.0);
            unsafe {
                fmod::FMOD_EventParameter_GetRange(param, &mut min, &mut max).as_result()?;
                fmod::FMOD_EventParameter_GetValue(param, &mut current).as_result()?;
            }
            param_list.push(FmodParam{
                name: convert_mem_to_string(param_name),
                range: (min, max),
                seekspeed: None,
                current: Some(current),
            });
        }
        let mut info = fmod::FMOD_EVENT_INFO::default();
        let mut state = 0;
        unsafe { fmod::FMOD_Event_GetInfo(event, null_mut(), null_mut(), &mut info).as_result()?; }
        unsafe { fmod::FMOD_Event_GetState(event, &mut state).as_result()?; }
        Ok(FmodPlayingEventInfo {
            // name: "TODO:".into(),
            playing: (state & 8) != 0,
            param_list, 
            positionms: info.positionms, 
            lengthms: info.lengthms
        })
    }

    pub fn get_event(&self, event_name: String, info: bool) -> FmodResult<*mut fmod::FMOD_EVENT> {
        let mode = if info { 4 } else { 0 };
        let mut event = null_mut();
        match CString::new(event_name) {
            Ok(s)=> {
                unsafe { fmod::FMOD_EventSystem_GetEvent(self.system, s.as_ptr(), mode, &mut event).as_result()?; }
                Ok(event)
            },
            Err(e)=> {
                Err(e.to_string())
            }
        }
    }

    pub fn get_info_by_id(&self, id: &String) -> FmodResult<JsonValue> {
        self.update()?;
        if let Some(event) = self.playing_events.get(id) {
            // let info = self.get_event_info(*event)?;
            let playing_info = self.get_playing_event_info(*event)?;
            Ok(playing_info.into())
        }
        else {
            Err(format!("Failed to get info, id not exists: {}", id))
        }
    }

    pub fn get_all_playing_info(&self) -> FmodResult<HashMap<String, JsonValue>> {
        self.update()?;
        let mut result = HashMap::with_capacity(self.playing_events.len());
        self.playing_events.iter()
            .for_each(|(id, event)|{
                let info = self.get_event_info(*event);
                let playing_info = self.get_playing_event_info(*event);
                if let (Ok(info), Ok(playing_info)) = (info, playing_info) {
                    let data = object! {
                        path: info.get_path(),
                        hash: info.get_hash(),
                        playing: playing_info.playing,
                        positionms: playing_info.positionms,
                        lengthms: playing_info.lengthms,
                        param_list: playing_info.param_list,
                    };
                    result.insert(id.to_string(), data);
                }
            });
        Ok(result)
    }

    pub fn play_sound(&mut self, event_name: String, id: String, params: Option<JsonValue>) -> FmodResult<()> {
        self.kill_sound(&id)?;
        let event = self.get_event(event_name, false)?;
        self.set_seek_mode(event, "instant")?;
        // ignore fadein time
        let mut fadein_time: i32 = 0;
        unsafe { fmod::FMOD_Event_SetPropertyByIndex(event, 
            fmod::FMOD_EVENT_PROPERTY_FMOD_EVENTPROPERTY_FADEIN as i32,
            &mut fadein_time as *mut i32 as *mut c_void, 1 /* this_instance */).as_result()?; }
        unsafe { fmod::FMOD_Event_Start(event).as_result()?; }
        self.playing_events.insert(id, event);
        
        // for `PlaySoundWithParams`
        if let Some(params) = params {
            let mut param  = null_mut();
            params.entries().for_each(|(k, v)|{
                let v = v.as_f32();
                if v.is_none() {
                    return;
                }
                if let Ok(s) = CString::new(k){
                    if unsafe { fmod::FMOD_Event_GetParameter(event, s.as_ptr(), &mut param).as_result() }.is_ok() {
                        let value = self.normalize_parameter_value(param, k, v.unwrap()).unwrap_or(0.0);
                        unsafe { fmod::FMOD_EventParameter_SetValue(param, value); }
                    }
                }
            });
        }
        Ok(())
    }

    pub fn kill_sound(&mut self, id: &String) -> FmodResult<()> {
        if let Some(event) = self.playing_events.remove(id) {
            if let Err(e) = unsafe { fmod::FMOD_Event_Stop(event, 1 /* true */).as_result() } {
                if e.as_str() != "FMOD_RESULT_FMOD_ERR_INVALID_HANDLE" { // ignore this warning, failed to kill sound is acceptable
                    eprintln!("[FMOD] Warning: failed to kill sound `{id}` ({e})");
                }
            }
        }
        Ok(())
    }

    pub fn set_parameter(&mut self, id: String, param_name: String, value: f32) -> FmodResult<()> {
        if let Some(event) = self.playing_events.get(&id) {
            let mut param = null_mut();
            unsafe { fmod::FMOD_Event_GetParameter(*event, param_name.clone().as_mut_ptr().cast(), &mut param).as_result()?; }
            let value = self.normalize_parameter_value(param, param_name.as_str(), value)?;
            unsafe { fmod::FMOD_EventParameter_SetValue(param, value).as_result()?; }
        }
        Ok(())
    }

    pub fn set_seek_mode(&self, event: *mut fmod::FMOD_EVENT, mode: &str) -> FmodResult<()> {
        let mut num_params = 0;
        unsafe { fmod::FMOD_Event_GetNumParameters(event, &mut num_params).as_result()?; }
        for i in 0..num_params {
            let mut param = null_mut();
            unsafe { fmod::FMOD_Event_GetParameterByIndex(event, i, &mut param).as_result()?; }
            match mode {
                "instant"=> unsafe {
                    fmod::FMOD_EventParameter_SetSeekSpeed(param, 1e10).as_result()?;
                },
                "seek"=> unsafe {
                    let default_speed = unimplemented!();
                    fmod::FMOD_EventParameter_SetSeekSpeed(param, default_speed).as_result()?;
                },
                _ => return Err(format!("Invalid seek mode: {}", mode))
            }
        }
        Ok(())
    }

    /// limit extreme parameter value
    /// for some sfx, increasing intensity above 95% may cause the sound effects to not play
    /// (unknown reason, I think there is a problem with the parameter range when klei made these sound effects)
    fn normalize_parameter_value(&self, param: *mut fmod::FMOD_EVENTPARAMETER, param_name: &str, value: f32) -> FmodResult<f32> {
        if param_name == "intensity" {
            let (mut min, mut max) = (0.0, 0.0);
            unsafe { fmod::FMOD_EventParameter_GetRange(param, &mut min, &mut max).as_result()?; }
            Ok(f32::min(value, max* 0.95))
        }
        else {
            Ok(value)
        }
    }

    pub fn set_global_volume(&mut self, volume: f32) -> FmodResult<()> {
        for (id, category) in &self.major_categories {
            if let Err(e) = unsafe { fmod::FMOD_EventCategory_SetVolume(*category, volume).as_result() } {
                println!("[FMOD] Failed to set volume to category: {}\n{}", id, e);
            }
        }
        self.volume = volume;
        Ok(())
    }

    pub fn get_global_volume(&self) -> FmodResult<f32> {
        Ok(self.volume)
    }

    pub fn update(&self) -> FmodResult<()> {
        match unsafe { fmod::FMOD_EventSystem_Update(self.system).as_result() } {
            Ok(_)=> Ok(()),
            Err(e)=> {
                eprintln!("[FMOD] Error in update: {}", e);
                std::process::exit(102);
            }
        }
    }
}