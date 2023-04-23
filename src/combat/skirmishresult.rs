use super::Attack;
pub use super::Warrior;

pub struct SkirmishResult {
    pub interaction_log: String,
    pub damage_dealt: i8,
}

impl SkirmishResult {
    pub fn new(attacker: &impl Warrior, attacked: &impl Warrior, attack: &Attack) -> Self {
        Self {
            interaction_log: format!(
                "{} attacked {} with {} - damage: {}",
                attacker.get_warrior_name(),
                attacked.get_warrior_name(),
                attack.attack_type.to_string(),
                attack.damage
            ),
            damage_dealt: attack.damage,
        }
    }
}
