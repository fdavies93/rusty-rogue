use std::{
    io::{self, Stdout},
    time::Duration, collections::{HashMap, HashSet}, hash::Hash,
};

use anyhow::Ok;
use serde::{Serialize, Deserialize};
use serde_json::Result;

use ratatui::{widgets::Paragraph, layout::Rect};

use crossterm::event::{KeyCode};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameEventType {
    GAME,
    INPUT
}

#[derive(Serialize, Deserialize)]
pub struct InputData {
    pub key_code: KeyCode
}

#[derive(Clone)]
// data is a JSON-encoded representation
pub struct GameEvent {
    pub ev_type: GameEventType,
    pub data: String
}

#[derive(Clone)]
pub struct GameEventQueue {
    next_id : u16,
    listeners: HashMap<u16, fn(&GameEvent, &mut GameObject, &TileMap)>,
    // attach listeners to event types
    listener_evs: HashMap<GameEventType, HashSet<u16>>
}

impl GameEventQueue {

    pub fn new() -> Self {
        Self {
            next_id: 0,
            listeners: HashMap::new(),
            listener_evs: HashMap::new()
        }
    }

    pub fn attach_listener(&mut self, func : fn(&GameEvent, &mut GameObject, &TileMap), listen_for : Vec<GameEventType>) -> u16 {        
        self.listeners.insert(self.next_id, func);
        
        for to_listen in listen_for {
            if !self.listener_evs.contains_key(&to_listen) {
                self.listener_evs.insert(to_listen, HashSet::new());
            }
            match self.listener_evs.get_mut(&to_listen) {
                None => panic!("Listener for this ev type doesn't exist."),
                Some(o) => {
                    o.insert(self.next_id);
                    ()
                }
            }
        }
        
        self.next_id += 1;
        return self.next_id - 1;
    }

    pub fn trigger_listeners(&mut self, ev: &GameEvent, caller: &mut GameObject, map: &TileMap) {
        let to_trigger: &mut HashSet<u16>;
        let type_of = ev.ev_type;
        match self.listener_evs.get_mut(&type_of) {
            None => return,
            Some(o) => {to_trigger = o} 
        }
        for id in to_trigger.iter() {
            match self.listeners.get(id) {
                None => panic!("Listeners by type and by index out of sync."),
                Some(o) => o(ev, caller, map)
            }
        }
    }
}

pub struct GameState {
    name: String,
    objects: HashMap<String, GameObject>
}

#[derive(Clone)]
pub struct GameObject {
    pub id: String,
    pub position: (u16, u16),
    pub glyph: char
    // pub listeners: HashMap<GameEventType, fn(&GameEvent, &mut GameObject, &TileMap)>
}

impl GameObject {
    pub fn to_text(&self) -> Paragraph {
        Paragraph::new(self.glyph.to_string())
    }

}

// pub struct GameManager {
//     pub event_queues : HashMap<String, GameEventQueue>
// }

// impl GameManager {
//     pub fn new() -> GameManager {
//         return Self {
//             event_queues: HashMap::new()
//         };
//     }
// }

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    FLOOR,
    WALL
}

#[derive(Clone)]
pub struct TileMap {
    tiles: Vec<Vec<TileType>>,
    size: (u16, u16)
}


impl TileMap {
    pub fn new(size : (u16, u16)) -> Self {
        Self {
            tiles: TileMap::instantiate_map(size),
            size: size
        }
    }

    pub fn instantiate_map ( size : (u16, u16) ) -> Vec<Vec<TileType>> {
        let mut tiles = Vec::new();

        for x in 0..size.0 {

            tiles.push(Vec::new());

            for _y in 0..size.1 {

                tiles[usize::from(x)].push(TileType::FLOOR);
            }
        }

        return tiles;
    }

    pub fn tile_at(&self, pos : (u16, u16)) -> TileType {
        return self.tiles[usize::from(pos.0)][usize::from(pos.1)];
    }

    pub fn get_size(&self) -> (u16, u16) {
        return self.size;
    }

    pub fn to_rect(&self) -> Rect {
        Rect { x: 0, y: 0, width: self.size.0, height: self.size.1 }
    }

    pub fn draw_rect(&mut self, pos: &Rect, tile: TileType, filled: bool) {
        // remove anything out of bounds of tilemap
        let real_pos = self.to_rect().intersection(*pos);

        for x in real_pos.left()..pos.right() {
            for y in real_pos.top()..pos.bottom() {
                if filled || (!filled && (
                    (x+1 == real_pos.right()) || 
                    (x == real_pos.left()) ||
                    (y+1 == real_pos.bottom()) ||
                    (y == real_pos.top())
                )) {
                    self.tiles[usize::from(x)][usize::from(y)] = tile
                }
            }
        }
    }

}

pub fn player_move(ev : &GameEvent, obj : &mut GameObject, map : &TileMap) {
    let data: InputData = serde_json::from_str(ev.data.as_str()).unwrap();
    let key = data.key_code;

    let mut destination = obj.position;

    if key == KeyCode::Left || key == KeyCode::Char('a') {
        destination.0 -= 1
    }
    else if key == KeyCode::Right || key == KeyCode::Char('d') {
        destination.0 += 1
    }
    else if key == KeyCode::Up || key == KeyCode::Char('w') {
        destination.1 -= 1
    }
    else if key == KeyCode::Down || key == KeyCode::Char('s') {
        destination.1 += 1
    }

    if map.tile_at(destination) == TileType::FLOOR {
        obj.position = destination;
    }

    return
}