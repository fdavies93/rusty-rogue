use crate::game::GameManager;
use crate::events::{GameEvent, Listener, InputData, HitData};
use crossterm::event::KeyCode;
use crate::components::{WorldPosition, TileMap, TileType, Vector2};

pub fn player_move(game: &mut GameManager, ev : &GameEvent, listener : &Listener) -> Vec<GameEvent> {
    let data: InputData = serde_json::from_str(ev.data.as_str()).unwrap();
    let key = data.key_code;

    let mut position: WorldPosition = game.get_component_data("WorldPosition", &listener.object_id).unwrap();
    
    if key == KeyCode::Left || key == KeyCode::Char('a') {
        position.x -= 1
    }
    else if key == KeyCode::Right || key == KeyCode::Char('d') {
        position.x += 1
    }
    else if key == KeyCode::Up || key == KeyCode::Char('w') {
        position.y -= 1
    }
    else if key == KeyCode::Down || key == KeyCode::Char('s') {
        position.y += 1
    }

    let world: TileMap = {
        let comps = &game.get_components_by_type_mut("TileMap").unwrap();
        let comp = &comps[0];
        serde_json::from_str(comp.data.as_str()).unwrap()
    };

    {
        let positions = game.get_components_by_type_mut("WorldPosition").unwrap();
        for comp in positions {
            let cur_pos: WorldPosition = serde_json::from_str(comp.data.as_str()).unwrap();
            if comp.obj_id != listener.object_id && cur_pos.x == position.x && cur_pos.y == position.y {
                // disallow move, but trigger an on_hit
                // println!("hit");

                let hit = GameEvent {
                    ev_type: "game.on_hit".to_string(),
                    data: serde_json::to_string(
                        &HitData {
                            aggressor: listener.object_id.clone(),
                            target: comp.obj_id.clone()
                        }
                    ).unwrap()
                };

                return vec![hit]
            }
        }        
    }

    let mut components = game.get_components("WorldPosition", &listener.object_id).unwrap();

    if world.tile_at(position.as_tuple_2()) != TileType::FLOOR {
        // disallow movement
        return vec![]
    }

    // finally move
    components[0].data = serde_json::to_string(&position).unwrap();

    return vec![]
}