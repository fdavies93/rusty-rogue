use serde::{Serialize, Deserialize};
use crate::Components::Component::IsComponent;
use std::str::FromStr;

#[derive(Serialize, Deserialize)]
pub struct Glyph {
    pub glyph: char
}

impl IsComponent for Glyph {
    fn get_type_name(&self) -> String {
        String::from_str("Glyph").unwrap()
    }
}