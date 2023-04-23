use super::Attack;
use super::SkirmishResult;

pub trait Warrior {
    fn get_warrior_name(&self) -> String;
    fn perform_attack_on(
        &mut self,
        opponent: &impl Warrior,
        attack: &Attack,
    ) -> SkirmishResult;
}