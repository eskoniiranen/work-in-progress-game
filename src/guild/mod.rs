use crate::character::Character;

pub struct Guild {
    pub name: String,
    pub members: Vec<Character>,
    pub max_size: usize,
}

impl Guild {
    pub fn new(name: &str) -> Self {
        Guild {
            name: name.to_string(),
            members: Vec::new(),
            max_size: 20,
        }
    }

    pub fn add_member(&mut self, character: Character) -> Result<(), String> {
        if self.members.len() >= self.max_size {
            Err(format!(
                "Guild '{}' is full (max {} members)",
                self.name, self.max_size
            ))
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
            Err(format!(
                "Character '{}' not found in guild '{}'",
                name, self.name
            ))
        }
    }

    pub fn list_members(&self) {
        for member in &self.members {
            println!("{}", member.name);
        }
    }

    pub fn is_full(&self) -> bool {
        self.members.len() >= self.max_size
    }

    pub fn size(&self) -> usize {
        self.members.len()
    }
}
