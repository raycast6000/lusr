use std::env;
use std::fs;
use std::{thread, time};
use rlua::{Lua, Context};

fn load_library(context: Context) {
    let to_json = context.create_function(|_, json: String| {
        println!("TO json: {}", json);
        Ok(())
    }).unwrap();

    let parse_json = context.create_function(|_, json: String| {
        let final_result: &str = "{test: 'Pretend this is real lol'}";
        println!("Parsing from JSON: {}", json);
        
        Ok(final_result)
    }).unwrap();

    let task_wait = context.create_function(|_, seconds: u64| {        
        thread::sleep(time::Duration::from_secs(seconds));
        Ok(true)
    }).unwrap();

    context.globals().set("toJson", to_json).unwrap();
    context.globals().set("parseJson", parse_json).unwrap();
    context.globals().set("wait", task_wait).unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let script = match fs::read_to_string(args[1].as_str()) {
        Ok(str) => str.as_str().to_owned(),
        Err(_) => {
            println!("[LUSR] Please provide a valid file. \nExample usage (the file is in the same directory): \nlusr main.lua");
            return;
        }
    };

    let lua_state: Lua = Lua::new();
    
    let result = lua_state.context(|context| {
        load_library(context);

        context.load(&script.to_string()).exec()
    });

    match result {
        Ok(_) => (),
        Err(e) => {
            println!("[LUSR] Whoops! Something went wrong.\n{}", e);
        }
    }

    println!("Used memory: {}", lua_state.used_memory());
}
