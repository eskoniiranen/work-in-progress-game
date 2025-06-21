use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, Serialize, Deserialize, Display, EnumString)]
#[serde(rename_all = "PascalCase")]
pub enum MeleeWeapon {
    Dagger,
    Sword,
    Axe,
    Mace,
    Greatsword,
    Greataxe,
    Maul,
}

#[derive(Debug, Clone, Serialize, Deserialize, Display, EnumString)]
#[serde(rename_all = "PascalCase")]
pub enum RangedWeapon {
    Sling,
    Bow,
    Crossbow,
}

#[derive(Debug, Clone, Serialize, Deserialize, Display, EnumString)]
#[serde(rename_all = "PascalCase")]
pub enum MagicalWeapon {
    Wand,
    Staff,
    Greatstaff,
}

#[derive(Debug, Clone, Serialize, Deserialize, Display, EnumString)]
#[serde(rename_all = "PascalCase")]
pub enum Offhand {
    Knife,
    ShortSword,
    Buckler,
    Shield,
    Tome,
    Skull,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemStats {
    pub damage: Option<u32>,
    pub magic_power: Option<u32>,
    pub defense: Option<u32>,
    pub crit_chance: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemEffect {
    IncreaseStat { stat: String, amount: i16 },
    GrantAbility(String),
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub kind: ItemKind,
    pub stats: ItemStats,
    pub effect: ItemEffect,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemKind {
    MeleeWeapon(MeleeWeapon),
    RangedWeapon(RangedWeapon),
    MagicalWeapon(MagicalWeapon),
    Offhand(Offhand),
}
