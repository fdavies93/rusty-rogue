use serde::{Serialize, Deserialize};
use crate::Components::Component::IsComponent;
use std::str::FromStr;

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

impl WorldPosition {
    pub fn as_tuple_2(&self) -> (u16, u16) {
        (self.x, self.y)
    }
}