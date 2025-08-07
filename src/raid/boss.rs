use common::position::Position;

#[derive(Debug)]
pub struct Boss {
    pub name: String,
    pub health: i32,
    pub position: Position,
    pub aoe_attacks: Vec<AoeAbility>,
    pub add_spawn_timer: u32,
    pub adds: Vec<Add>,
}

#[derive(Debug)]
pub struct Add {
    pub name: String,
    pub health: i32,
    pub position: Position,
}

pub struct AoeAbility {}
