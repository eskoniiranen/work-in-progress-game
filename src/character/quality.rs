use rand::distributions::WeightedIndex;
use rand::prelude::*;

pub const STANDARD: i32 = 10;
pub const UNCOMMON: i32 = 12;
pub const RARE: i32 = 14;
pub const EPIC: i32 = 16;

#[derive(Debug, Clone, Copy)]
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
    pub fn value(&self) -> i32 {
        match self {
            Quality::Standard => STANDARD,
            Quality::Uncommon => UNCOMMON,
            Quality::Rare => RARE,
            Quality::Epic => EPIC,
        }
    }
}
