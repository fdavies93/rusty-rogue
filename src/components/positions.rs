use serde::{Serialize, Deserialize};
use crate::components::IsComponent;
use std::str::FromStr;

pub trait Vector2 {
    fn as_tuple_2(&self) -> (u16, u16);
}

#[derive(Serialize, Deserialize)]
pub struct WorldPosition {
    pub x: u16,
    pub y: u16,
    // this one is the id of the tilemap component
    pub map: u16 
}

impl IsComponent for WorldPosition {
    fn get_type_name(&self) -> String {
        String::from_str("WorldPosition").unwrap()
    }
}

impl Vector2 for WorldPosition {
    fn as_tuple_2(&self) -> (u16, u16) {
        (self.x, self.y)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ScreenPosition {
    pub x: u16,
    pub y: u16,
}

impl IsComponent for ScreenPosition {
    fn get_type_name(&self) -> String {
        String::from_str("ScreenPosition").unwrap()
    }
}

impl Vector2 for ScreenPosition {
    fn as_tuple_2(&self) -> (u16, u16) {
        (self.x, self.y)
    }
}