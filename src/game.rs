use std::{
    io::{self, Stdout},
    time::Duration, collections::HashMap,
};

use ratatui::{widgets::Paragraph as Paragraph, layout::Rect};

pub struct GameState {
    name: String,
    objects: HashMap<String, GameObject>
}

#[derive(Debug, Clone, Copy)]
pub struct GameObject {
    pub position: (u16, u16),
    pub glyph: char
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