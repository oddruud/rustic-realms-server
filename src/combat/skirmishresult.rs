pub use super::Combatant;


pub struct SkirmishResult<'a> {
    pub interaction_log : String,
    pub damage_dealt : i32,
    pub attacked: &'a dyn Combatant,
}

impl<'a> SkirmishResult<'a> {
    pub fn new(attacked: &'a impl Combatant) -> Self{
        Self{attacked: attacked, interaction_log: "".to_string(), damage_dealt:0}
    }
}