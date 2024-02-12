use std::{thread, time};
use mlua::{Lua, Table};

// I know this code sucks. But at least it works lol 

fn register_lib(lua_state: &Lua, lib: Vec<&str>) {
    for api in lib {
        lua_state.globals().set(api, lua_state.create_table().unwrap()).unwrap()
    }
}

pub fn load_libraries(lua_state: &Lua) {
    let to_json = lua_state.create_function(|_, obj: Table| {
        let js = serde_json::to_string(&obj).unwrap();

        Ok(js)
    }).unwrap();

    let parse_json = lua_state.create_function(|_, _json: String| {
        Ok("Not available yet.")
    }).unwrap();

    
    let task_wait = lua_state.create_function(|_, seconds: u32| {        
        thread::sleep(time::Duration::from_secs(seconds.into()));
        Ok(true)
    }).unwrap();

    register_lib(&lua_state, vec!["thread", "json"]);

    let thread_gl: Table = lua_state.globals().get("thread").unwrap();
    let json_gl: Table = lua_state.globals().get("json").unwrap();

    thread_gl.set("wait", task_wait).unwrap();
    json_gl.set("parse", parse_json).unwrap();
    json_gl.set("toJSON", to_json).unwrap();
}