#[derive(Debug)]
pub enum RaidControl {
    MassCooldownReduction { duration: u32, amount: u32 },
    MassHeal { amount: i32 },
    Retarget { new_priority: TargetType },
}

#[derive(Debug, Clone, Copy)]
pub enum TargetType {
    Boss,
    Adds,
}
