use super::super::Edible;
use super::super::EdibleType;
use crate::combat::{Warrior, SkirmishResult, AttackType};

pub struct Courgette {
    pub name:String,
    pub edible_type:EdibleType,
}

impl Courgette {
    pub fn new(name:String) -> Self{
        Self{name: name, edible_type: EdibleType::Vegetable}
    }
}

impl Edible for Courgette {
    fn get_name(&self)-> &str{
        &self.name
    }

    fn get_type(&self)-> &EdibleType{
        &self.edible_type
    }
}

impl Warrior for Courgette  {
    fn get_warrior_name(& self)-> String {
        format!("{} the provocateur",self.name)
    }

    fn perform_attack_on(&mut self, opponent: &impl Warrior, attack_type: AttackType) -> SkirmishResult {
        let damage = 20;
        return SkirmishResult::new( self, opponent, attack_type, damage);
    }
}