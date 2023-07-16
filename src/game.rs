use std::{
    io::{self, Stdout},
    time::Duration, collections::{HashMap, HashSet}, hash::Hash, str::FromStr,
};

use anyhow::Ok;
use serde::{Serialize, Deserialize};
use serde_json::Result;
use std::mem::transmute_copy;

use ratatui::{widgets::{Paragraph, List}, layout::Rect};

use crossterm::event::{KeyCode};

#[derive(Serialize, Deserialize)]
pub struct InputData {
    pub key_code: KeyCode
}


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

#[derive(Clone, Serialize, Deserialize)]
pub struct TileMap {
    tiles: Vec<Vec<TileType>>,
    size: (u16, u16)
}

impl IsComponent for TileMap {
    fn get_type_name(&self) -> String {
        String::from_str("TileMap").unwrap()
    }
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

#[derive(Serialize, Deserialize)]
pub struct WorldPosition {
    pub x: u16,
    pub y: u16,
    // this one is the id of the tilemap component
    pub map: u16 
}

impl IsComponent for WorldPosition {
    fn get_type_name(&self) -> String {
        String::from_str("WorldPosition").unwrap()
    }
}

impl WorldPosition {
    pub fn as_tuple_2(&self) -> (u16, u16) {
        (self.x, self.y)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Glyph {
    pub glyph: char
}

impl IsComponent for Glyph {
    fn get_type_name(&self) -> String {
        String::from_str("Glyph").unwrap()
    }
}

pub trait IsComponent {
    fn get_type_name(&self) -> String;
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
    pub object_id: String,
    pub to_trigger: fn(&mut GameManager, &GameEvent, &Listener)
}

impl Listener {
pub fn new (listen_for: Vec<String>, object_id: &str, to_trigger: fn(&mut GameManager, &GameEvent, &Listener)) -> Self {
        Self {
            id: 0,
            listen_for,
            object_id: String::from_str(object_id).unwrap(),
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
        let listen_for = &self.listeners.get(&self.next_id).unwrap().listen_for;
        
        for to_listen in listen_for {
            if !self.listener_evs.contains_key(to_listen) {
                self.listener_evs.insert(to_listen.clone(), HashSet::new());
            }
            match self.listener_evs.get_mut(to_listen) {
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
        let type_of = &ev.ev_type;
        match self.listener_evs.get_mut(type_of) {
            None => return,
            Some(o) => {to_trigger = o} 
        }
        for id in to_trigger.iter() {
            match self.listeners.get(id) {
                None => panic!("Listeners by type and by index out of sync."),
                Some(o) => (o.to_trigger)(game, ev, o)
            }
        }
    }
}

pub struct GameState {
    name: String,
}

// all event queues are stored by *object*, which seems wrong
pub struct GameManager {
    next_id: u16,
    // event_queue : GameEventQueue,
    components : HashMap<u16, Component>,
    components_by_type : HashMap<String, HashSet<u16>>,
    components_by_obj : HashMap<String, HashSet<u16>>
}

impl GameManager {
    pub fn new() -> GameManager {
        return Self {
            next_id: 0,
            // event_queue: GameEventQueue::new(),
            components: HashMap::new(),
            components_by_obj: HashMap::new(),
            components_by_type: HashMap::new()
        };
    }

    // pub fn add_listener(&mut self, mut to_attach: Listener) { 
    //     self.event_queue.attach_listener(to_attach);
    // }

    // pub fn trigger_listeners(&mut self, ev: &GameEvent) {
    //     // this is difficult to work around without changing signature of
    //     // trigger_listeners
    //     let eq = {
    //         &mut self.event_queue
    //     };

    //     eq.trigger_listeners(self, ev);
    // }

    pub fn add_component(&mut self, component: Component) -> u16 {
    
        self.components.insert(self.next_id, component);
        let component = &self.components[&self.next_id];
        // add id to hashset if hashset exists, else create it
        let set = match self.components_by_obj.get_mut(&component.obj_id) {
            None => {
                self.components_by_obj.insert(component.obj_id.clone(), HashSet::new());
                self.components_by_obj.get_mut(&component.obj_id).unwrap()
            }
            Some(o) => o
        };
        set.insert(self.next_id);

        let set = match self.components_by_type.get_mut(&component.c_type) {
            None => {
                self.components_by_type.insert(component.c_type.clone(), HashSet::new());
                self.components_by_type.get_mut(&component.c_type).unwrap()
            }
            Some(o) => o
        };
        set.insert(self.next_id);

        // add 
        self.next_id += 1;
        self.next_id - 1
    }

    pub fn add_component_from_data(&mut self, datum: &(impl IsComponent + Serialize), obj_id: &str) -> u16 {
        let mut comp = Component::new(obj_id.to_string());
        comp.set_data(datum);
        self.add_component(comp)
    }

    pub fn get_component_mut(&mut self, id: u16) -> Option<&mut Component> {
        self.components.get_mut(&id)
    }

    pub fn get_component(&self, id: u16) -> Option<&Component> {
        self.components.get(&id)
    }

    // pub fn get_components_by_obj_mut<'a>(&'a mut self, obj: &str) -> impl Iterator<Item = &'a mut Component> + 'a {
    //     self.components_by_obj.get(obj).into_iter().flatten().map(|id| {
    //         self.components.get_mut(id).unwrap()
    //     })
    // }

    pub fn get_components_by_obj_mut(&mut self, obj: &str) -> Option<Vec<&mut Component>> {
        match self.components_by_obj.get(obj) {
            None => Option::None,
            Some(id_set) => {

                let ids = Vec::from_iter(id_set);

                let mut components: Vec<*mut Component> = vec![];
                let comp_link = &mut self.components;
                for id in ids {
                    {
                    // we want this to panic on fail because it means
                    // the indexes have gone out of sync
                    let comp: *mut Component = comp_link.get_mut(id).unwrap();
                    components.push(comp);
                    }
                }
                // hack to allow mapping hashset to hashmap keys
                // very bad and literally unsafe, high priority to refactor
                let components = components.into_iter().map(|ptr| unsafe { &mut *ptr } ).collect();
                Option::Some( components )
            }
        }
    }

    pub fn get_components_by_type_mut(&mut self, c_type: &str) -> Option<Vec<&mut Component>> {
        match self.components_by_type.get(c_type) {
            None => Option::None,
            Some(ids) => {
                let mut components: Vec<*mut Component> = vec![];
                let comp_link = &mut self.components;
                for id in ids {
                    // we want this to panic on fail because it means
                    // the indexes have gone out of sync
                    components.push(comp_link.get_mut(id).unwrap());
                }
                // very bad and literally unsafe, high priority to refactor
                let components = components.into_iter().map(|ptr| unsafe { &mut *ptr } ).collect();

                Option::Some(components)
            }
        }
    }

    pub fn get_components(&mut self, c_type: &str, obj: &str) -> Option<Vec<&mut Component>> {
        let by_type = self.components_by_type.get(c_type);
        let by_obj = self.components_by_obj.get(obj);
        if by_type.is_none() || by_obj.is_none() {
            Option::None
        }
        else {
            let by_type = by_type.unwrap();
            let by_obj = by_obj.unwrap();
            let mut union: Vec<*mut Component> = vec![];
            for id in by_type.intersection(by_obj) {
                // we want this to panic on fail because it means
                // the indexes have gone out of sync
                union.push(self.components.get_mut(id).unwrap())
            }
            // very bad and literally unsafe, high priority to refactor
            let union: Vec<&mut Component> = union.into_iter().map(|ptr| unsafe { &mut *ptr } ).collect();
            Option::Some(union)
        }
    }

}

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TileType {
    FLOOR,
    WALL
}


pub fn player_move(game: &mut GameManager, ev : &GameEvent, listener : &Listener) {
    let data: InputData = serde_json::from_str(ev.data.as_str()).unwrap();
    let key = data.key_code;

    let mut position: WorldPosition = {
        let components = game.get_components("WorldPosition", &listener.object_id).unwrap();
        serde_json::from_str(components[0].data.as_str()).unwrap()
    };

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

    let mut components = game.get_components("WorldPosition", &listener.object_id).unwrap();

    if world.tile_at(position.as_tuple_2()) == TileType::FLOOR {
        // allow movement
        components[0].data = serde_json::to_string(&position).unwrap();
    }

    return
}