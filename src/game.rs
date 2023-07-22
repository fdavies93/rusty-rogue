use std::collections::{HashMap, HashSet};

use serde::Serialize;
use crate::Components::Component::{Component, IsComponent};
use crate::events::GameEvent;

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