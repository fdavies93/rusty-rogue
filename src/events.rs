use serde::{Serialize, Deserialize};
use crossterm::event::KeyCode;
use crate::game::GameManager;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Serialize, Deserialize)]
pub struct InputData {
    pub key_code: KeyCode
}

#[derive(Serialize, Deserialize)]
pub struct HitData {
    pub aggressor: String,
    pub target: String
}

#[derive(Serialize, Deserialize)]
pub struct TickData {
    pub tick: u16
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
    pub to_trigger: fn(&mut GameManager, &GameEvent, &Listener) -> Vec<GameEvent>
}

impl Listener {
pub fn new (listen_for: Vec<&str>, object_id: &str, to_trigger: fn(&mut GameManager, &GameEvent, &Listener) -> Vec<GameEvent>) -> Self {
        let mut lf: Vec<String> = vec![];

        for a in listen_for {
            lf.push(String::from_str(a).unwrap());
        }

        Self {
            id: 0,
            listen_for: lf,
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

    pub fn trigger_listeners(&mut self, game: &mut GameManager, initial_ev: GameEvent) {
        let mut evs = vec![initial_ev];

        while evs.len() > 0 {

            let ev = evs.pop().unwrap();

            if ev.ev_type == "game.remove_object" {
                game.remove_object(ev.data.as_str());
            }

            let to_trigger: &mut HashSet<u16>;
            let type_of = ev.ev_type.as_str();
            match self.listener_evs.get_mut(type_of) {
                None => return,
                Some(o) => {to_trigger = o} 
            }
            for id in to_trigger.iter() {
                let mut callbacks : Vec<GameEvent> = match self.listeners.get(id) {
                    None => panic!("Listeners by type and by index out of sync."),
                    Some(o) => (o.to_trigger)(game, &ev, o)
                };
                evs.append(&mut callbacks);
            };
        }

        
    }
}