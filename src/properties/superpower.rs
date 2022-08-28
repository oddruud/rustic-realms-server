use std::fmt;

pub struct SuperPower {
    superpower_type: SuperPowerType,
    proficiency: i32,
}

impl SuperPower {
    pub fn new(power_type: SuperPowerType) -> Self {
        Self {
            superpower_type: power_type,
            proficiency: 0,
        }
    }
}

impl fmt::Display for SuperPower {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "POWER: {} - level: {}",
            self.superpower_type, self.proficiency
        )
    }
}

#[derive(strum_macros::Display)]
pub enum SuperPowerType {
    Flight,
    LaserEyes,
    Teleportation,
}
