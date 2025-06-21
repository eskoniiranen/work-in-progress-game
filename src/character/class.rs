use rand::Rng;
use rand::distributions::{Distribution, Standard};
use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Display)]
#[serde(rename_all = "PascalCase")]
pub enum Class {
    Thief,
    Ranger,
    Wizard,
    Warlock,
    Warrior,
    Paladin,
    Cleric,
    Bard,
}

impl Distribution<Class> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Class {
        use Class::*;
        match rng.gen_range(0..=12) {
            0 => Thief,
            1 => Ranger,
            2 => Wizard,
            3 => Warlock,
            4 => Warrior,
            5 => Paladin,
            6 => Cleric,
            _ => Bard,
        }
    }
}
