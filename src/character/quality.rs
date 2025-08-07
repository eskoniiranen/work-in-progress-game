use rand::distributions::WeightedIndex;
use rand::prelude::*;
use serde::{Deserialize, Serialize};

pub const STANDARD: i16 = 10;
pub const UNCOMMON: i16 = 12;
pub const RARE: i16 = 14;
pub const EPIC: i16 = 16;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum Quality {
    Standard,
    Uncommon,
    Rare,
    Epic,
}

pub fn get_random_quality(rng: &mut impl Rng) -> Quality {
    let qualities = [
        Quality::Standard,
        Quality::Uncommon,
        Quality::Rare,
        Quality::Epic,
    ];
    let weights = [50, 30, 15, 5];

    let dist = WeightedIndex::new(weights).unwrap();
    let index = dist.sample(rng);
    qualities[index]
}

impl Quality {
    pub fn value(&self) -> i16 {
        match self {
            Quality::Standard => STANDARD,
            Quality::Uncommon => UNCOMMON,
            Quality::Rare => RARE,
            Quality::Epic => EPIC,
        }
    }
}
