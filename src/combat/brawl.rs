
use serde::Deserialize;
use serde::Serialize;

use crate::properties::SuperPower;
use crate::properties::SuperPowerType;

use crate::food::fruit::mango::Mango;
use crate::food::vegetables::courgette::Courgette;
use crate::combat::Attack;
use crate::combat::AttackType;
use crate::combat::Warrior;

#[derive(Debug, Deserialize, Serialize)]
pub struct Brawl{
    pub fight_name : String,
    pub fighter1 : String,
    pub fighter2 : String,
}

impl Brawl
{

    pub fn new(fight_name: String, fighter1: String, fighter2: String) -> Self { 
        Self {fight_name: fight_name, fighter1: fighter1, fighter2: fighter2}
    }

    pub fn test_brawl (&mut self) -> String 
    {
        let pow: SuperPower = SuperPower::new(SuperPowerType::LaserEyes);
    
        //BILLY THE MANGO
        let mut billy_the_mango = Mango::new("Billy".to_string());
        billy_the_mango.set_super_power(pow);
        billy_the_mango.who_are_you();
      
        //ROSS THE COURGETTE
        let mut ross_the_courgette: Courgette = Courgette::new("Ross".to_string());
    
        let verbal_attack = Attack::new(AttackType::Verbal("YEHAAA".to_owned(), 20), "Verbal attack".to_owned(), 10);
        let whimsical_attack = Attack::new(AttackType::Whimsical("BOINK".to_owned(), 20), "whiimsical".to_owned(), 10);
    
        //BATTLE!!!!
        let ross_attacks = ross_the_courgette.perform_attack_on(&billy_the_mango, &verbal_attack);
        let billy_attacks = billy_the_mango.perform_attack_on(&ross_the_courgette, &whimsical_attack);
    
        return format!("{} {}", ross_attacks.interaction_log, billy_attacks.interaction_log);
    }
    
}
