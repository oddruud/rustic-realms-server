use super::EdibleType;

pub trait Edible {
    fn get_health(&self) -> i32;
    fn set_health(&mut self, health:i32);
    fn get_name(&self) -> &str;
    fn get_type(&self) -> &EdibleType;
}