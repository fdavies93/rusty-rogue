use anyhow::{Context, Result, Error};
use game::{GameObject, TileMap, TileType};
use ratatui::{backend::CrosstermBackend, widgets::Paragraph, Terminal, layout::Rect};
use std::{
    io::{self, Stdout},
    time::Duration, collections::HashMap,
};
use crossterm::{
    event::{self, Event, KeyCode},
};

mod rterm;
mod game;

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
        glyph: '@'
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

        let mut playerObj: &mut GameObject;
        match objects.get_mut("player") {
            None => panic!("Couldn't find player object."),
            Some(player) => playerObj = player
        }
        let mut destination = playerObj.position;

        if key == KeyCode::Esc {
            break;
        }
        else if key == KeyCode::Left {
            destination.0 -= 1
        }
        else if key == KeyCode::Right {
            destination.0 += 1
        }
        else if key == KeyCode::Up {
            destination.1 -= 1
        }
        else if key == KeyCode::Down {
            destination.1 += 1
        }
        
        if map.tile_at(destination) == TileType::FLOOR {
            playerObj.position = destination;
        }
    }
    Ok(())
}