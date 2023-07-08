use std::str::FromStr;

use js_sys::JsString;
use log::{info};
use screeps::{game, raw_memory, SharedCreepProperties};
use serde_json::Result;


use crate::my_memory::MyMemory;

pub fn get_memory() -> MyMemory {
    let raw_memory_str = raw_memory::get().as_string().unwrap();
    info!("{}", raw_memory_str);
    let res: Result<MyMemory> = serde_json::from_str(raw_memory_str.as_str());
    match res {
        Ok(my_memory) => my_memory,
        _ => MyMemory::default(),
    }
}

pub fn set_memory(my_memory: &MyMemory) {
    let serialized = serde_json::to_string(my_memory).unwrap();
    raw_memory::set(
        &JsString::from_str(serialized.as_str()).unwrap(),
    );
}
