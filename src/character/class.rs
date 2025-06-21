use rand::Rng;
use rand::seq::SliceRandom;
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

pub fn all_classes() -> &'static [Class] {
    use Class::*;
    &[
        Thief, Ranger, Wizard, Warlock, Warrior, Paladin, Cleric, Bard,
    ]
}

pub fn random_class<R: Rng + ?Sized>(rng: &mut R) -> Class {
    *all_classes().choose(rng).unwrap()
}
