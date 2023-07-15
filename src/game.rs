use std::{
    io::{self, Stdout},
    time::Duration, collections::{HashMap, HashSet}, hash::Hash,
};

use anyhow::Ok;
use serde::{Serialize, Deserialize};
use serde_json::Result;

use ratatui::{widgets::{Paragraph, List}, layout::Rect};

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


// A Component represents any component of a GameObject.
pub struct Component {
    pub obj_id: String,
    // A serialisable struct e.g. TileMap, Camera
    pub data: String
}

#[derive(Clone)]
// data is a JSON-encoded representation
pub struct GameEvent {
    pub ev_type: String,
    pub data: String
}

#[derive(Clone)]
pub struct Listener {
    pub id: u16,
    // use ev_type to deliver system events
    // e.g. game.close, input.remap
    pub listen_for: Vec<String>,
    pub subject_id: u16,
    pub to_trigger: fn(&mut GameManager, &GameEvent)
}

impl Listener {
pub fn new (listen_for: Vec<String>, subject_id: u16, to_trigger: fn(&mut GameManager, &GameEvent)) -> Self {
        Self {
            id: 0,
            listen_for,
            subject_id,
            to_trigger
        }
    }
}

#[derive(Clone)]
pub struct GameEventQueue {
    next_id : u16,
    // hash id of listener against listener function
    listeners: HashMap<u16, Listener>,
    // hash event types against listener ids
    listener_evs: HashMap<String, HashSet<u16>>
}

impl GameEventQueue {

    pub fn new() -> Self {
        Self {
            next_id: 0,
            listeners: HashMap::new(),
            listener_evs: HashMap::new()
        }
    }

    pub fn attach_listener(&mut self, mut to_attach : Listener) -> u16 {   
        to_attach.id = self.next_id;
        self.listeners.insert(self.next_id, to_attach);
        
        for to_listen in to_attach.listen_for {
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

    pub fn trigger_listeners(&mut self, game: &mut GameManager, ev: &GameEvent) {
        let to_trigger: &mut HashSet<u16>;
        let type_of = ev.ev_type;
        match self.listener_evs.get_mut(&type_of) {
            None => return,
            Some(o) => {to_trigger = o} 
        }
        for id in to_trigger.iter() {
            match self.listeners.get(id) {
                None => panic!("Listeners by type and by index out of sync."),
                Some(o) => (o.to_trigger)(game, ev)
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

// all event queues are stored by *object*, which seems wrong
pub struct GameManager {
    event_queues : HashMap<String, GameEventQueue>,
    pub objects : HashMap<String, GameObject>,
    pub components : HashMap<String, Component>
}

impl GameManager {
    pub fn new() -> GameManager {
        return Self {
            event_queues: HashMap::new(),
            objects: HashMap::new(),
            components: HashMap::new()
        };
    }

    fn add_event_queue(&mut self, obj_id: String) -> &mut GameEventQueue {
        let eq = GameEventQueue::new();
        self.event_queues.insert(obj_id.clone(), eq);
        match self.event_queues.get_mut(&obj_id) {
            None => panic!("Insert of new value failed somehow."),
            Some(eq) => eq
        }
    }

    fn add_listener(&mut self, obj_id: String, f: fn(&mut GameManager, &GameEvent, &mut GameObject), listen_for: Vec<GameEventType>) { 
        let eq = match self.event_queues.get_mut(&obj_id) {
            None => self.add_event_queue(obj_id),
            Some(q) => q
        };
        // eq.attach_listener(f, listen_for);
    }

}

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TileType {
    FLOOR,
    WALL
}

#[derive(Clone, Serialize, Deserialize)]
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