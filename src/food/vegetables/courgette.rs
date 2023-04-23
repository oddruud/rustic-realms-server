use super::super::Edible;
use super::super::EdibleType;

pub struct Courgette {
    pub name:String,
    pub edible_type:EdibleType,
    pub health:i32,
}

impl Courgette {
    pub fn new(name:String) -> Self{
        Self{name: name, edible_type: EdibleType::Vegetable, health: 100}
    }
}

impl Edible for Courgette {
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