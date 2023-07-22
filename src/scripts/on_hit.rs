use crate::game::GameManager;
use crate::events::{GameEvent, Listener, InputData};
use crossterm::event::KeyCode;
use crate::components::{WorldPosition, TileMap, TileType};

pub fn on_hit(game: &mut GameManager, ev : &GameEvent, listener : &Listener) -> Vec<GameEvent> {
    return vec![]
}