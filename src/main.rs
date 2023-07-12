use anyhow::{Context, Result, Error};
use game::{GameObject, TileMap, TileType, GameEventType, GameEvent, InputData, GameEventQueue};
use ratatui::{backend::CrosstermBackend, widgets::{Paragraph, canvas::Map}, Terminal, layout::Rect};
use std::{
    io::{self, Stdout},
    time::Duration, collections::HashMap,
};
use crossterm::{
    event::{self, Event, KeyCode},
};

mod rterm;
mod game;

use game::player_move;

/// This is a bare minimum example. There are many approaches to running an application loop, so
/// this is not meant to be prescriptive. It is only meant to demonstrate the basic setup and
/// teardown of a terminal application.
///
/// A more robust application would probably want to handle errors and ensure that the terminal is
/// restored to a sane state before exiting. This example does not do that. It also does not handle
/// events or update the application state. It just draws a greeting and exits when the user
/// presses 'q'.
fn main() -> Result<()> {
    let mut player = GameObject {
        position: (1,1),
        glyph: '@',
        events: GameEventQueue::new()
    };
    let mut terminal = rterm::setup_terminal().context("setup failed")?;
    let mut objs = HashMap::from([
        ("player".to_string(), player),
    ]);
    // ratatui handles text overflowing the buffer by truncating it - good
    // translating from world -> camera space should therefore be sufficient
    // for rendering to succeed even on large maps
    let mut map = TileMap::new( (15,15) );
    map.draw_rect(&Rect { x: 0, y: 0, width: 15, height: 15 }, TileType::WALL, false);
    map.draw_rect(&Rect { x: 6, y: 6, width: 3, height: 3 }, TileType::WALL, true);
    
    run(&mut terminal, &mut objs, &map).context("app loop failed")?;
    rterm::restore_terminal(&mut terminal).context("restore terminal failed")?;
    
    Ok(())
}

// Render and poll terminal for keypress events
pub fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>, objects : &mut HashMap<String, GameObject>, map : &TileMap) -> Result<()> {
    
    loop {
        terminal.draw(rterm::assemble_render(objects, map))?;
        let key = rterm::poll()?;
        let input_ev = GameEvent {
            ev_type: GameEventType::INPUT,
            data: serde_json::to_string( &InputData {
                key_code: key
            })?
        };

        if key == KeyCode::Esc { break }
        for obj in objects.values_mut() {
            let queue = &mut obj.events;
            queue.trigger_listeners(&input_ev, obj, map);         
        }
    
    }
    Ok(())
}