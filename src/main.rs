use food::Mango;
use properties::SuperPower;
use properties::SuperPowerType;
use properties::Emotion;
use errors::RequestError;

mod properties;
mod food;
mod errors;

fn main() {
    let pow : SuperPower = SuperPower::new(SuperPowerType::LaserEyes);

    let mut billy_the_mango = Mango::new(String::from("Billy"));
    billy_the_mango.set_power(pow);
    billy_the_mango.who_are_you();
    
    let emotion = billy_the_mango.are_you_happy().unwrap_or(Emotion::Sad);
    println!("its seems that {} is {}", billy_the_mango.name, emotion);
}
