use ratatui::text::Text;

use std::stringify;
use crate::game::GameManager;
use crate::events::{GameEvent, Listener, TickData};
use crate::components::{ScreenPosition, Monitor, TextBox, Vector2, Health};

pub fn update_health(game: &mut GameManager, ev : &GameEvent, listener : &Listener) -> Vec<GameEvent> {
    // we don't care about the event as it holds no useful info
    // this should be the component

    let monitor : Monitor = match game.get_component_data("Monitor", &listener.object_id) {
        None => return vec![],
        Some(c) => c
    };
    
    let health_str : String = {

        let obj_id = monitor.to_monitor.iter().find(|p| -> bool { p.1 == "Health" });

        let obj_id = match obj_id {
            None => return vec![],
            Some(c) => c
        };


        let comps = match game.get_components("Health", obj_id.0.as_str()) {
            None => return vec![],
            Some(c) => c
        };

        let mut hs = format!("?/?");

        if comps.len() > 0 {
            let health : Health = serde_json::from_str(comps[0].data.as_str()).unwrap();
            hs = format!("{}/{}", health.current_health, health.max_health);
        }
        
        hs
    };
    
    let tb = TextBox {
        value: health_str
    };

    let mut comp = match game.get_components("TextBox", listener.object_id.as_str()) {
        Some(c) => c,
        None => return vec![]
    };
    comp[0].data = serde_json::to_string(&tb).unwrap();

    return vec![];
}