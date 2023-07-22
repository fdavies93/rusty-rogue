use serde::{Serialize, Deserialize};
use crate::Components::Component::IsComponent;
use std::str::FromStr;

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