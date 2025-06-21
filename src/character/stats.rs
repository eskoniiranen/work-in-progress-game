use crate::character::Quality;

use super::race::Race;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub strength: i32,
    pub dexterity: i32,
    pub wisdom: i32,
    pub tenacity: i32,
}

const HIGH: i32 = 3;
const NORMAL: i32 = 0;
const LOW: i32 = -3;

pub fn get_quality_stats(quality: Quality) -> Stats {
    Stats {
        strength: quality.value(),
        dexterity: quality.value(),
        wisdom: quality.value(),
        tenacity: quality.value(),
    }
}

pub fn get_race_base_stats(race: &Race) -> Stats {
    use Race::*;

    match race {
        Human => Stats {
            strength: NORMAL,
            dexterity: NORMAL,
            wisdom: NORMAL,
            tenacity: NORMAL,
        },
        Troll => Stats {
            strength: HIGH,
            dexterity: LOW,
            wisdom: NORMAL,
            tenacity: NORMAL,
        },
        Orc => Stats {
            strength: HIGH,
            dexterity: NORMAL,
            wisdom: LOW,
            tenacity: NORMAL,
        },
        Leonin => Stats {
            strength: HIGH,
            dexterity: NORMAL,
            wisdom: NORMAL,
            tenacity: LOW,
        },
        Elf => Stats {
            strength: LOW,
            dexterity: HIGH,
            wisdom: NORMAL,
            tenacity: NORMAL,
        },
        Centaur => Stats {
            strength: NORMAL,
            dexterity: HIGH,
            wisdom: LOW,
            tenacity: NORMAL,
        },
        Harpy => Stats {
            strength: NORMAL,
            dexterity: HIGH,
            wisdom: NORMAL,
            tenacity: LOW,
        },
        Halfling => Stats {
            strength: LOW,
            dexterity: NORMAL,
            wisdom: HIGH,
            tenacity: NORMAL,
        },
        Draconic => Stats {
            strength: NORMAL,
            dexterity: LOW,
            wisdom: HIGH,
            tenacity: NORMAL,
        },
        Gnome => Stats {
            strength: NORMAL,
            dexterity: NORMAL,
            wisdom: HIGH,
            tenacity: LOW,
        },
        Goblin => Stats {
            strength: LOW,
            dexterity: NORMAL,
            wisdom: NORMAL,
            tenacity: HIGH,
        },
        Dwarf => Stats {
            strength: NORMAL,
            dexterity: LOW,
            wisdom: NORMAL,
            tenacity: HIGH,
        },
        Minotaur => Stats {
            strength: NORMAL,
            dexterity: NORMAL,
            wisdom: LOW,
            tenacity: HIGH,
        },
    }
}

pub fn get_stats(race: &Race, quality: Quality) -> Stats {
    get_race_base_stats(race).combine(&get_quality_stats(quality))
}

impl Stats {
    pub fn combine(&self, other: &Stats) -> Stats {
        Stats {
            strength: (self.strength + other.strength),
            dexterity: (self.dexterity + other.dexterity),
            wisdom: (self.wisdom + other.wisdom),
            tenacity: (self.tenacity + other.tenacity),
        }
    }
}
