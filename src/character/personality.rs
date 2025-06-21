use rand::Rng;
use serde::{Deserialize, Serialize};

use super::quality::Quality;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Personality {
    pub optimism: i32,
    pub altruism: i32,
    pub reflexes: i32,
    pub teamwork: i32,
    pub awareness: i32,
    pub focus: i32,
}

pub fn get_personality<R: Rng + ?Sized>(rng: &mut R, quality: Quality) -> Personality {
    Personality {
        optimism: quality.value() + rng.gen_range(-5..=5),
        altruism: quality.value() + rng.gen_range(-5..=5),
        reflexes: quality.value() + rng.gen_range(-5..=5),
        teamwork: quality.value() + rng.gen_range(-5..=5),
        awareness: quality.value() + rng.gen_range(-5..=5),
        focus: quality.value() + rng.gen_range(-5..=5),
    }
}
