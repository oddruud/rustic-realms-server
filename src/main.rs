use food::Mango;
use food::Courgette;
use superpower::SuperPower;
use superpower::SuperPowerType;

mod superpower;
mod food;


fn main() {
    let pow : SuperPower = SuperPower::new(SuperPowerType::LaserEyes);

    let mut billy_the_mango = Mango::new(String::from("Billy"));
    billy_the_mango.set_power(pow);
    billy_the_mango.who_are_you();    
}
