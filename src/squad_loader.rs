use screeps::Room;

use crate::loop_contex::LoopContext;

pub trait Squad {
    fn squad_name(&self) -> String;
    fn is_active(&self) -> bool;
    fn operate(&self);
}

pub trait PerRoomSquad: Squad {
    fn new(loop_context: &mut LoopContext, room: &Room) -> Self;
}

pub fn per_room_squads(_loop_context: &LoopContext, _room: &Room) -> &'static [&'static dyn Squad] {
    &[]
}
