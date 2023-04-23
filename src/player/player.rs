use serde::Serialize;
use serde::Deserialize;


#[derive(Debug, Deserialize, Serialize)]
pub struct Player {
    pub name : String,
}

impl Player {
    pub fn new(name: String) -> Self { 
        Self {name: name}
    }
}