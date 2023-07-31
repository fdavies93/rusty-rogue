use serde::{Serialize, Deserialize};
use crate::components::IsComponent;
use std::str::FromStr;

// A monitor is an information component which helps you fetch data about
// some other component. It's intended for use with UI components which
// display information e.g. a health bar.
#[derive(Serialize, Deserialize)]
pub struct Monitor {
    // object name, component type
    pub to_monitor: Vec<(String, String)>
}

impl IsComponent for Monitor {
    fn get_type_name(&self) -> String {
        String::from_str("Monitor").unwrap()
    }
}
