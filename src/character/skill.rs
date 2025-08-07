#[derive(Debug)]
pub struct Skill {
    pub name: String,
    pub range: f32,
    pub cooldown: u32,
    pub current_cooldown: u32,
    pub effect: SkillEffect,
}

#[derive(Debug)]
pub enum SkillEffect {
    Damage(i32),
    Heal(i32),
    BuffCooldownReduction(u32), // e.g. reduce cooldowns by 5s
}
