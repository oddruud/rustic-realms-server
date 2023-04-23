use super::super::{Edible, EdibleType};
use crate::properties::SuperPower;

pub struct Mango {
    pub name : String,
    pub super_power : Option<SuperPower>,
    pub edible_type: EdibleType,
    pub health: i32
}

impl Mango {
    pub fn new(name: String) -> Self { 
        Self {name: name, super_power: Option::None, edible_type: EdibleType::Fruit, health: 100}
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
}

impl Edible for Mango {
    fn get_name(&self)-> &str{
        &self.name
    }

    fn get_type(&self)-> &EdibleType{
        &self.edible_type
    }

    fn get_health(&self) -> i32 {
        self.health
    }

    fn set_health(&mut self, health:i32){
        self.health = health
    }
}
