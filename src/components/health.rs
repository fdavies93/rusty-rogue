use serde::{Serialize, Deserialize};
use crate::components::IsComponent;
use std::str::FromStr;

#[derive(Serialize, Deserialize)]
pub struct Health {
    pub current_health: u16,
    pub max_health: u16
}

impl IsComponent for Health {
    fn get_type_name(&self) -> String {
        String::from_str("Health").unwrap()
    }
}

#[derive(Serialize, Deserialize)]
pub struct HealthMonitor {
    pub subject_id: String
}

impl IsComponent for HealthMonitor {
    fn get_type_name(&self) -> String {
        String::from_str("HealthMonitor").unwrap()
    }
}