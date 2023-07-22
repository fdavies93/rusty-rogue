use serde::Serialize;

// A Component represents any component of a GameObject.
pub struct Component {
    pub id: u16,
    pub obj_id: String,
    // A serialisable struct e.g. TileMap, Camera
    pub data: String,
    pub c_type: String
}

impl Component {
    pub fn new(obj_id: String) -> Self {
        Self {
            id: 0,
            obj_id,
            data: String::new(),
            c_type: String::new()
        }
    }

    pub fn set_data(&mut self, item: &(impl IsComponent + Serialize)) {
        self.c_type = item.get_type_name();
        self.data = serde_json::to_string(item).unwrap();
    }
}
pub trait IsComponent {
    fn get_type_name(&self) -> String;
}