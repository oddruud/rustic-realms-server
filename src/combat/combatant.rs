use super::SkirmishResult; 
use super::AttackType;


pub trait Combatant {
    fn perform_attack_on(&mut self, opponent: &impl Combatant, attackType: AttackType) -> SkirmishResult {
        let mut result = SkirmishResult::new(opponent);
        result.interaction_log = "a battle took place between SELF AND OPPONENT".to_string();
        return result;
    }
}