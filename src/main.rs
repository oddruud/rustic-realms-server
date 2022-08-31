#![allow(dead_code)]

use food::Courgette;
use food::Mango;

use properties::Emotion;
use properties::SuperPower;
use properties::SuperPowerType;

use combat::AttackType;
use combat::Warrior;

mod combat;
mod errors;
mod food;
mod properties;

fn main() {
    let pow: SuperPower = SuperPower::new(SuperPowerType::LaserEyes);

    //BILLY THE MANGO
    let mut billy_the_mango = Mango::new("Billy".to_string());
    billy_the_mango.set_super_power(pow);
    billy_the_mango.who_are_you();
    let emotion = billy_the_mango.are_you_happy().unwrap_or(Emotion::Sad);
    println!("its seems that {} is {}", billy_the_mango.name, emotion);

    //ROSS THE COURGETTE
    let mut ross_the_courgette: Courgette = Courgette::new("Ross".to_string());

    //BATTLE!!!!
    let ross_attacks = ross_the_courgette.perform_attack_on(&billy_the_mango, AttackType::Verbal);
    let billy_attacks = billy_the_mango.perform_attack_on(&ross_the_courgette, AttackType::Whimsical);
    
    println!("a fight took place:");
    println!("{}", ross_attacks.interaction_log);
    println!("{}", billy_attacks.interaction_log);
}
