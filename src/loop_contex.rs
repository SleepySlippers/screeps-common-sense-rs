use std::cell::RefCell;

use screeps::{game, Room};

use crate::{
    api_call::{get_memory, set_memory},
    my_memory::MyMemory,
};

pub struct LoopContext {
    pub memory: RefCell<MyMemory>,
    pub visible_rooms: Vec<Room>,
}

impl LoopContext {
    pub fn new() -> LoopContext {
        LoopContext {
            memory: RefCell::new(get_memory()),
            visible_rooms: game::rooms().values().collect(),
        }
    }
}

impl Drop for LoopContext {
    fn drop(&mut self) {
        set_memory(&self.memory.borrow());
    }
}
