use crate::combat::Combatant;

pub struct Courgette {
    pub name:String,
}

impl Courgette {
    pub fn new(name:String) -> Self{
        Self{name: name}
    } 
}

impl Combatant for Courgette {
    
}
