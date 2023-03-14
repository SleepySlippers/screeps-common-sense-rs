use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RoomInfo {}

#[derive(Serialize, Deserialize)]
pub struct CreepState {}

#[derive(Serialize, Deserialize)]
pub struct CreepSettings {}

#[derive(Serialize, Deserialize)]
pub struct CreepInfo {
    state: CreepState,
    settings: CreepSettings,
}

#[derive(Serialize, Deserialize, Default)]
pub struct MyMemory {
    pub rooms: HashMap<String, RoomInfo>,
    pub creeps: HashMap<String, CreepInfo>,
}
