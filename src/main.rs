use anyhow::{Context, Result};
use game::{GameObject, TileMap, TileType};
use ratatui::{backend::CrosstermBackend, widgets::Paragraph, Terminal, layout::Rect};
use std::{
    io::{self, Stdout},
    time::Duration, collections::HashMap,
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
    let player = GameObject {
        position: (1,1),
        glyph: '@'
    };
    let mut terminal = rterm::setup_terminal().context("setup failed")?;
    let objs = HashMap::from([
        ("player".to_string(), player),
    ]);
    // ratatui handles text overflowing the buffer by truncating it - good
    // translating from world -> camera space should therefore be sufficient
    // for rendering to succeed even on large maps
    let mut map = TileMap::new( (15,15) );
    map.draw_rect(&Rect { x: 0, y: 0, width: 15, height: 15 }, TileType::WALL, false);
    run(&mut terminal, &objs, &map).context("app loop failed")?;
    rterm::restore_terminal(&mut terminal).context("restore terminal failed")?;
    Ok(())
}

/// Run the application loop. This is where you would handle events and update the application
/// state. This example exits when the user presses 'q'. Other styles of application loops are
/// possible, for example, you could have multiple application states and switch between them based
/// on events, or you could have a single application state and update it based on events.
pub fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>, objects : &HashMap<String, GameObject>, map : &TileMap) -> Result<()> {
    loop {
        terminal.draw(rterm::assemble_render(objects, map))?;
        if rterm::should_quit()? {
            break;
        }
    }
    Ok(())
}