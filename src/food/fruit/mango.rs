use crate::superpower::SuperPower;

pub struct Mango {
    pub name : String,
    pub super_power : Option<SuperPower>,
}

impl Mango {
    pub fn new(name: String) -> Self { 
        Self {name: name, super_power: Option::None}
    }

    pub fn set_power(&mut self, power : SuperPower){
        self.super_power = Option::Some(power);
    }
    
    pub fn who_are_you(&self){

        match &self.super_power {
            Option::None => println!("Im {0}, the mango! I have no superpower :(", self.name),
            Option::Some(p) => println!("Im {0}, the mango! my super power is {1}", self.name, p),
        }

        
    }
}
