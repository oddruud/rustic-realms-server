use super::EdibleType;

pub trait Edible {
    fn get_name(&self) -> &str;
    fn get_type(&self) -> &EdibleType;
}

