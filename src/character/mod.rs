pub mod abilities;
pub mod class;
pub mod inventory;
pub mod item;
pub mod personality;
pub mod quality;
pub mod race;
pub mod skill;
pub mod stats;

use abilities::get_class_abilities;
use class::Class;
use inventory::Inventory;
use personality::Personality;
use quality::Quality;
use quality::get_random_quality;
use race::Race;
use stats::Stats;
use stats::get_stats;

use serde::{Deserialize, Serialize};

use crate::{common::position::Position, raid::{boss::Boss, raidcontrol::TargetType}};

use self::{personality::get_personality, skill::Skill};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub class: Class,
    pub race: Race,
    pub quality: Quality,
    pub personality: Personality,
    pub inventory: Inventory,
    pub abilities: Vec<String>,
    pub stats: Stats,
}

impl Character {
    pub fn new(name: &str) -> Self {
        let mut rng = rand::thread_rng();
        let quality: Quality = get_random_quality(&mut rng);
        let race: Race = rand::random();
        let class: Class = rand::random();

        Character {
            name: name.to_string(),
            class,
            race,
            quality,
            personality: get_personality(&mut rng, quality),
            inventory: Inventory::default(),
            abilities: get_class_abilities(&class),
            stats: get_stats(&race, quality),
        }
    }
}

#[derive(Debug)]
pub struct CharacterState {
    pub character: Character,
    pub position: Position,
    pub skills: Vec<Skill>,
    pub target_priority: TargetType,
    pub current_target: Option<Boss>,
}
