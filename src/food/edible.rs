use super::EdibleType;
use crate::combat::*;

pub trait Edible {
    fn get_health(&self) -> i32;
    fn set_health(&mut self, health:i32);
    fn get_name(&self) -> &str;
    fn get_type(&self) -> &EdibleType;
}

impl<T> Warrior for T where T: Edible {
    fn get_warrior_name(&self) -> String {
        self.get_name().to_owned()
    }

    fn perform_attack_on(
        &mut self,
        opponent: &impl Warrior,
        attack: &Attack,
    ) -> SkirmishResult {
        let mut damage = 20;

        match attack.attack_type {
            AttackType::Whimsical(_, power) => damage = power,
            AttackType::Verbal(_, power) => damage = power,
            _ => damage = 0,
        }
        
        return SkirmishResult::new(self, opponent, &attack);
    }
}
