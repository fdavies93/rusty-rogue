use anyhow::{Context, Result, Error};
use game::{TileMap, TileType, GameEvent, InputData, GameEventQueue, GameManager, Listener, Component};
use ratatui::{backend::CrosstermBackend, widgets::{Paragraph, canvas::Map}, Terminal, layout::Rect};
use std::{
    io::{self, Stdout},
    time::Duration, collections::HashMap, str::FromStr,
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
    let mut terminal = rterm::setup_terminal().context("setup failed")?;
    
    let mut player_pos = game::WorldPosition {
        x: 1,
        y: 1,
        map: 0
    };

    let mut player_glyph = game::Glyph {
        glyph: '@'
    };

    let mut enemy_pos = game::WorldPosition {
        x: 10,
        y: 10,
        map: 0
    };

    let mut enemy_glyph = game::Glyph {
        glyph: 'M'
    };

    // ratatui handles text overflowing the buffer by truncating it - good
    // translating from world -> camera space should therefore be sufficient
    // for rendering to succeed even on large maps
    let mut map = TileMap::new( (15,15) );
    map.draw_rect(&Rect { x: 0, y: 0, width: 15, height: 15 }, TileType::WALL, false);
    map.draw_rect(&Rect { x: 6, y: 6, width: 3, height: 3 }, TileType::WALL, true);

    let mut game = GameManager::new();

    game.add_component_from_data(&player_pos, "player");
    game.add_component_from_data(&player_glyph, "player");
    game.add_component_from_data(&map, "map");
    game.add_component_from_data(&enemy_glyph, "enemy");
    game.add_component_from_data(&enemy_pos, "enemy");

    let mut eq = GameEventQueue::new();

    let lis = Listener::new(
        vec![String::from_str("input.key_press").unwrap()], 
        "player", 
        game::player_move
    );

    eq.attach_listener(lis);

    run(&mut terminal, &mut game, &mut eq).context("app loop failed")?;
    rterm::restore_terminal(&mut terminal).context("restore terminal failed")?;

    Ok(())
}

// Render and poll terminal for keypress events
pub fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>, game : &mut GameManager, eq : &mut GameEventQueue) -> Result<()> {
    loop {
        terminal.draw(rterm::assemble_render(game))?;
        let key = rterm::poll()?;
        let input_ev = GameEvent {
            ev_type: "input.key_press".to_string(),
            data: serde_json::to_string( &InputData {
                key_code: key
            })?
        };

        if key == KeyCode::Esc { break }
        eq.trigger_listeners(game, &input_ev)
    
    }
    Ok(())
}