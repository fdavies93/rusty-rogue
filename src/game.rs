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

pub trait ToText {
    fn to_text(&self) -> Paragraph;
}

impl ToText for GameObject {
    fn to_text(&self) -> Paragraph {
        Paragraph::new(self.glyph.to_string())
    }
}