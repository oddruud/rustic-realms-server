use super::AttackType;

pub struct Attack
{
    pub attack_type : AttackType,
    pub description : String,
    pub damage : i8,
}

impl Attack {
    pub fn new(a_type: AttackType, description : String, damage : i8) -> Self { 
        Self {attack_type : a_type, description : description , damage : damage}
    }   
}
