pub mod abilities;
pub mod class;
pub mod inventory;
pub mod item;
pub mod personality;
pub mod quality;
pub mod race;
pub mod stats;

use abilities::get_class_abilities;
use class::Class;
use class::random_class;
use inventory::Inventory;
use personality::Personality;
use quality::Quality;
use quality::get_random_quality;
use race::Race;
use race::random_race;
use stats::Stats;
use stats::get_stats;

use serde::{Deserialize, Serialize};

use self::personality::get_personality;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub class: Class,
    pub race: Race,
    pub personality: Personality,
    pub inventory: Inventory,
    pub abilities: Vec<String>,
    pub stats: Stats,
}

impl Character {
    pub fn new(name: &str) -> Self {
        let mut rng = rand::thread_rng();
        let quality: Quality = get_random_quality(&mut rng);
        let race: Race = random_race(&mut rng);
        let class: Class = random_class(&mut rng);

        Character {
            name: name.to_string(),
            class,
            race,
            personality: get_personality(&mut rng, quality),
            inventory: Inventory::default(),
            abilities: get_class_abilities(&class),
            stats: get_stats(&race, quality),
        }
    }
}
