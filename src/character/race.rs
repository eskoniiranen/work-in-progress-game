use rand::Rng;
use rand::seq::SliceRandom;
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

pub fn all_races() -> &'static [Race] {
    use Race::*;
    &[
        Human, Troll, Orc, Leonin, Elf, Centaur, Harpy, Halfling, Draconic, Gnome, Goblin, Dwarf,
        Minotaur,
    ]
}

pub fn random_race<R: Rng + ?Sized>(rng: &mut R) -> Race {
    *all_races().choose(rng).unwrap()
}
