use std::env;
use std::fs;
use mlua::prelude::*;

use crate::libs::load_libraries;

mod libs;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => panic!("\nAn input file is required. For example:\n\n$ lusr myscript.lua\n\n"),
        _ => {}
    };

    let script = match fs::read_to_string(args[1].as_str()) {
        Ok(str) => str.as_str().to_owned(),
        Err(_) => {
            println!("[CLSTR] Please provide a valid file. \nExample usage (the file is in the same directory): \ncluster main.lua");
            return;
        }
    };

    let lua_state: Lua = Lua::new();
    load_libraries(&lua_state);

    lua_state.load(script)
                    .set_name("LUSR")
                    .exec().unwrap();

    println!("Used memory: {}", lua_state.used_memory());
}
