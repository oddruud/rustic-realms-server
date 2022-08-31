use super::super::{Edible, EdibleType};
use crate::combat::{Warrior, SkirmishResult};

use crate::properties::SuperPower;
use crate::properties::Emotion;
use crate::errors::RequestError;

pub struct Mango {
    pub name : String,
    pub super_power : Option<SuperPower>,
    pub edible_type: EdibleType
}

impl Mango {
    pub fn new(name: String) -> Self { 
        Self {name: name, super_power: Option::None, edible_type: EdibleType::Fruit}
    }

    pub fn set_super_power(&mut self, power : SuperPower){
        self.super_power = Option::Some(power);
    }

    pub fn who_are_you(&self) {
        match &self.super_power {
            Option::None => println!("Im {0}, the mango! I have no superpower :(", self.name),
            Option::Some(p) => println!("Im {0}, the mango! my super power is {1}", self.name, p),
        }
    }

    pub fn are_you_happy(&self) -> Result<Emotion, RequestError> {

        //TODO randomization

        return Result::Ok(Emotion::Confused);
    }
}

impl Warrior for Mango {
    fn get_warrior_name(& self)-> String {
        format!("{} the juicy",self.name)
    }

    fn perform_attack_on(&mut self, opponent: &impl Warrior, attack_type: crate::combat::AttackType) -> crate::combat::SkirmishResult {
        
        let mut damage = 20;

        if self.super_power.is_some() {
            damage *= 100;
        }

        return SkirmishResult::new(self, opponent, attack_type, damage);
    }

}


impl Edible for Mango {
    fn get_name(&self)-> &str{
        &self.name
    }

    fn get_type(&self)-> &EdibleType{
        &self.edible_type
    }
}
