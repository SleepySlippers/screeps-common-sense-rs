use std::str::FromStr;

use js_sys::JsString;
use log::{info};
use screeps::{game, raw_memory, SharedCreepProperties};
use serde_json::Result;


use crate::my_memory::MyMemory;

pub fn get_memory() -> MyMemory {
    let raw_memory_str = raw_memory::get().as_string().unwrap();
    info!("{}", raw_memory_str);
    let keys = game::creeps().keys();
    info!("hehe {}", keys.collect::<Vec<String>>().join(","));
    let values = game::creeps().values();
    info!(
        "lulu {}",
        values.map(|c| c.name()).collect::<Vec<String>>().join(",")
    );
    let res: Result<MyMemory> = serde_json::from_str(raw_memory_str.as_str());
    match res {
        Ok(my_memory) => my_memory,
        _ => MyMemory::default(),
    }
}

pub fn set_memory(my_memory: &MyMemory) {
    raw_memory::set(
        &JsString::from_str(serde_json::to_string(my_memory).unwrap().as_str()).unwrap(),
    );
}
