use std::{
    io::{self, Stdout},
    time::Duration, collections::HashMap,
};

use ratatui::widgets::Paragraph as Paragraph;

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

#[derive(Clone, Copy)]
pub enum TileType {
    FLOOR,
    WALL
}

#[derive(Clone)]
pub struct TileMap {
    tiles: Vec<Vec<TileType>>,
    size: (usize, usize)
}


impl TileMap {
    pub fn new(size : (usize, usize)) -> Self {
        Self {
            tiles: TileMap::instantiate_map(size),
            size: size
        }
    }

    pub fn instantiate_map ( size : (usize, usize) ) -> Vec<Vec<TileType>> {
        let mut tiles = Vec::new();

        for x in 0..size.0 {

            tiles.push(Vec::new());

            for _y in 0..size.1 {

                tiles[x].push(TileType::FLOOR);
            }
        }

        return tiles;
    }

    pub fn tile_at(&self, pos : (usize, usize)) -> TileType {
        return self.tiles[pos.0][pos.1];
    }

    pub fn get_size(&self) -> (usize, usize) {
        return self.size;
    }
}