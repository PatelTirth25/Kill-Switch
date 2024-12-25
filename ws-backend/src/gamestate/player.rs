use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Player {
    pub id: String,
    pub x: i32,
    pub y: i32,
    pub weapon: bool,
    pub die: bool,
}

impl Player {
    pub fn new(id: String, x: i32, y: i32, weapon: bool, die: bool) -> Player {
        Player {
            id,
            x,
            y,
            weapon,
            die,
        }
    }
}
