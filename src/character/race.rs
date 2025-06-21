use rand::Rng;
use rand::distributions::{Distribution, Standard};
use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Debug, Clone, Copy, Display, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Race {
    Human,
    Troll,
    Orc,
    Leonin,
    Elf,
    Centaur,
    Harpy,
    Halfling,
    Draconic,
    Gnome,
    Goblin,
    Dwarf,
    Minotaur,
}

impl Distribution<Race> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Race {
        use Race::*;
        match rng.gen_range(0..=12) {
            0 => Human,
            1 => Troll,
            2 => Orc,
            3 => Leonin,
            4 => Elf,
            5 => Centaur,
            6 => Harpy,
            7 => Halfling,
            8 => Draconic,
            9 => Gnome,
            10 => Goblin,
            11 => Dwarf,
            _ => Minotaur,
        }
    }
}
