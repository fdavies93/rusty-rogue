use crate::game::GameManager;
use crate::events::{GameEvent, Listener, InputData, HitData};
use crossterm::event::KeyCode;
use crate::components::{WorldPosition, TileMap, TileType, Health};

pub fn on_hit(game: &mut GameManager, ev : &GameEvent, listener : &Listener) -> Vec<GameEvent> {
    
    let hit_data : HitData = serde_json::from_str(ev.data.as_str()).unwrap();
    
    if hit_data.target != listener.object_id {
        // i.e. - is it me?
        return vec![]        
    }

    let mut health: Health = match game.get_component_data("Health", &hit_data.target) {
        None => return vec![],
        Some(c) => c
    };

    health.current_health -= 1;

    if health.current_health == 0 {
        return vec![
            GameEvent {
                ev_type: "game.remove_object".to_string(),
                data: listener.object_id.clone()
            }
        ];
    }
    
    let mut components = game.get_components("Health", &hit_data.target).unwrap();

    components[0].data = serde_json::to_string(&health).unwrap();

    return vec![]
}