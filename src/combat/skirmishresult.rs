use super::AttackType;
pub use super::Warrior;

pub struct SkirmishResult {
    pub interaction_log: String,
    pub damage_dealt: i32,
}

impl SkirmishResult {
    pub fn new(attacker: &impl Warrior, attacked: &impl Warrior, attack_type: AttackType, damage: i32) -> Self {
        Self {
            interaction_log: format!(
                "{} attacked {} with {} - damage: {}",
                attacker.get_warrior_name(),
                attacked.get_warrior_name(),
                attack_type, 
                damage
            ),
            damage_dealt: damage,
        }
    }
}
