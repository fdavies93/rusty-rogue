use std::{
    io::{self, Stdout},
    time::Duration, collections::HashMap, hash::Hash,
};

use anyhow::Ok;
use serde::{Serialize, Deserialize};
use serde_json::Result;

use ratatui::{widgets::Paragraph, layout::Rect};

use crossterm::event::{KeyCode};

#[derive(Clone, PartialEq, Eq, Hash)]
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
pub struct GameState {
    name: String,
    objects: HashMap<String, GameObject>
}

#[derive(Clone)]
pub struct GameObject {
    pub position: (u16, u16),
    pub glyph: char,
    pub listeners: HashMap<GameEventType, fn(&GameEvent, &mut GameObject, &TileMap)>
}

impl GameObject {
    pub fn to_text(&self) -> Paragraph {
        Paragraph::new(self.glyph.to_string())
    }
}

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