use ratatui::text::Text;

use std::stringify;
use crate::game::GameManager;
use crate::events::{GameEvent, Listener, TickData};
use crate::components::{ScreenPosition, HealthMonitor, TextBox, Vector2, Health};

pub fn update_health(game: &mut GameManager, ev : &GameEvent, listener : &Listener) -> Vec<GameEvent> {
    // we don't care about the event as it holds no useful info
    // this should be the component
    
    let health_monitor : HealthMonitor = {
        let comps = match game.get_components("HealthMonitor", &listener.object_id) {
            None => return vec![],
            Some(c) => c
        };
        serde_json::from_str(comps[0].data.as_str()).unwrap()
    };

    
    let health_str : String = {
        let comps = match game.get_components("Health", &health_monitor.subject_id) {
            None => vec![],
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

    let mut comp = game.get_components("TextBox", listener.object_id.as_str()).unwrap();
    comp[0].data = serde_json::to_string(&tb).unwrap();

    return vec![];
}