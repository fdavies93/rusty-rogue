use serde::{Serialize, Deserialize};
use crate::components::IsComponent;
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

#[derive(Serialize, Deserialize)]
pub struct TextBox {
    pub value: String
}

impl IsComponent for TextBox {
    fn get_type_name(&self) -> String {
        String::from_str("TextBox").unwrap()
    }
}