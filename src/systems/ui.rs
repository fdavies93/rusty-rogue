use ratatui::text::Text;

use crate::components::{
    Component, Health, IsComponent, Monitor, ScreenPosition, TextBox, Vector2,
};
use crate::events::{GameEvent, Listener, TickData};
use crate::game::GameManager;
use std::stringify;

pub fn on_update(game: &mut GameManager, ev: &GameEvent) {
    // get all Monitor components
    let monitors: Vec<&mut Component> = match game.get_components_by_type_mut("Monitor") {
        None => return,
        Some(c) => c,
    };

    // HEALTH FUNCTION BEGINS HERE
    // collect monitors by attribute being monitored
    let health_monitors: Vec<&Health> = monitors
        .iter()
        .fold(|c| {
            let monitor : Monitor = c.extract_data();
            for to_monitor in monitor.to_monitor {
                
            }
        })
        .collect();

    // get the actual health object (will be in separate function later)
    let health_comps = health_monitors.iter_mut().fold( vec![], | acc, el | {
        
    });


    let comps = vec![];
    for comp in monitors {
        // find ui components on objects
        let textboxes = match game.get_components_by_obj_mut("TextBox") {
            // if there isn't one, skip
            None => continue,
            Some(c) => c,
        };
    }

    // deliver to appropriate subsystems -> extract renderable info e.g. max_val, cur_val
    // then deliver the render info to UI components of objects
    // right now, only TextBox is a valid UI component
    // TextBox needs better config for rendering e.g. template string
    // or another component that formats text

    let mut get_str = || -> String {
        let mut hs = format!("?/?");

        let monitor: Monitor = match game.get_component_data("Monitor", &listener.object_id) {
            None => return hs,
            Some(c) => c,
        };

        let obj_id = monitor
            .to_monitor
            .iter()
            .find(|p| -> bool { p.1 == "Health" });

        let obj_id = match obj_id {
            None => return hs,
            Some(c) => c,
        };

        let comps = match game.get_components("Health", obj_id.0.as_str()) {
            None => return hs,
            Some(c) => c,
        };

        if comps.len() > 0 {
            let health: Health = serde_json::from_str(comps[0].data.as_str()).unwrap();
            hs = format!("{}/{}", health.current_health, health.max_health);
        }

        hs
    };

    let health_str: String = get_str();

    let tb = TextBox { value: health_str };

    let mut comp = match game.get_components("TextBox", listener.object_id.as_str()) {
        Some(c) => c,
        None => return vec![],
    };

    comp[0].data = serde_json::to_string(&tb).unwrap();

    return vec![];
}
