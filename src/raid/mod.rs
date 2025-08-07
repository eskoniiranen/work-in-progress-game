use crate::character::{Character, class::Class};
use crate::common::position::Position;
use crate::guild::Guild;

pub mod boss;
pub mod raidcontrol;

#[derive(Debug, Clone)]
pub struct Raid {
    pub name: String,
    pub members: Vec<Character>,
    pub max_size: usize,
}

impl Raid {
    pub fn new(name: &str) -> Self {
        Raid {
            name: name.to_string(),
            members: Vec::new(),
            max_size: 10,
        }
    }

    pub fn from_guild(guild: &Guild, member_names: &[&str]) -> Self {
        let mut raid = Raid::new(&format!("{} Raid", guild.name));

        for name in member_names {
            if let Some(member) = guild.members.iter().find(|c| c.name == *name) {
                if raid.members.len() < raid.max_size {
                    raid.members.push(member.clone());
                }
            }
        }

        raid
    }

    pub fn add_member(&mut self, character: Character) -> Result<(), String> {
        if self.members.len() >= self.max_size {
            Err("Raid is full".to_string())
        } else {
            self.members.push(character);
            Ok(())
        }
    }

    pub fn remove_member_by_name(&mut self, name: &str) -> Result<(), String> {
        if let Some(pos) = self.members.iter().position(|c| c.name == name) {
            self.members.remove(pos);
            Ok(())
        } else {
            Err(format!("No character named '{}' in raid", name))
        }
    }

    pub fn get_tanks(&self) -> Vec<&Character> {
        self.members
            .iter()
            .filter(|c| matches!(c.class, Class::Warrior | Class::Paladin))
            .collect()
    }

    pub fn get_support(&self) -> Vec<&Character> {
        self.members
            .iter()
            .filter(|c| matches!(c.class, Class::Cleric | Class::Bard))
            .collect()
    }

    pub fn get_dps(&self) -> Vec<&Character> {
        self.members
            .iter()
            .filter(|c| {
                matches!(
                    c.class,
                    Class::Thief | Class::Ranger | Class::Wizard | Class::Warlock
                )
            })
            .collect()
    }
}
