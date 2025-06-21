use super::class::Class;

pub fn get_class_abilities(class: &Class) -> Vec<String> {
    use Class::*;
    match class {
        Thief => vec!["Backstab", "Steal", "Vanish"],
        Ranger => vec!["Snipe", "Trap", "Animal Bond"],
        Wizard => vec!["Fireball", "Teleport", "Mana Shield"],
        Warlock => vec!["Raise Dead", "Drain Life", "Dark Pact"],
        Warrior => vec!["Slash", "Charge", "Battle Cry"],
        Paladin => vec!["Smite", "Heal", "Divine Shield"],
        Cleric => vec!["Bless", "Cure", "Holy Light"],
        Bard => vec!["Inspire", "Lullaby", "Mocking Tune"],
    }
    .iter()
    .map(|s| s.to_string())
    .collect()
}
