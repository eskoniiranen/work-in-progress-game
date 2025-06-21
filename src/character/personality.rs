use rand::Rng;
use serde::{Deserialize, Serialize};

use super::quality::Quality;

const RNG_VARIANCE: i16 = 5;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Personality {
    pub optimism: i16,
    pub altruism: i16,
    pub reflexes: i16,
    pub teamwork: i16,
    pub awareness: i16,
    pub focus: i16,
}

fn get_variance<R: Rng + ?Sized>(rng: &mut R) -> i16 {
    rng.gen_range(-RNG_VARIANCE..=RNG_VARIANCE)
}

pub fn get_personality<R: Rng + ?Sized>(rng: &mut R, quality: Quality) -> Personality {
    Personality {
        optimism: quality.value() + get_variance(rng),
        altruism: quality.value() + get_variance(rng),
        reflexes: quality.value() + get_variance(rng),
        teamwork: quality.value() + get_variance(rng),
        awareness: quality.value() + get_variance(rng),
        focus: quality.value() + get_variance(rng),
    }
}
