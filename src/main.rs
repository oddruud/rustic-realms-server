#![allow(dead_code)]

use food::Mango;
use food::Courgette;

use properties::SuperPower;
use properties::SuperPowerType;
use properties::Emotion;

use combat::Combatant;
use combat::AttackType;

mod properties;
mod food;
mod errors;
mod combat;

fn main() {
    let pow : SuperPower = SuperPower::new(SuperPowerType::LaserEyes);

    //BILLY THE MANGO
    let mut billy_the_mango = Mango::new("Billy".to_string());
    billy_the_mango.set_power(pow);
    billy_the_mango.who_are_you();    
    let emotion = billy_the_mango.are_you_happy().unwrap_or(Emotion::Sad);
    println!("its seems that {} is {}", billy_the_mango.name, emotion);

    //ROSS THE COURGETTE
    let mut ross_the_courgette:Courgette = Courgette::new("Ross".to_string());
    
    //BATTLE!!!!
    let result = billy_the_mango.perform_attack_on(&ross_the_courgette, AttackType::Whimsical);

    println!("a fight took place:\n {}", result.interaction_log);
}
