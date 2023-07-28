use anyhow::{Context, Result, Error};

use game::GameManager;
use events::{GameEvent, Listener, GameEventQueue, InputData, TickData};
use components::{WorldPosition, Glyph, TileMap, TileType, Health, TextBox, ScreenPosition, Monitor};
use scripts::{player_move, on_hit, update_health};

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
mod components;
mod events;
mod scripts;

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
    
    let player_pos = WorldPosition {
        x: 1,
        y: 1,
        map: 0
    };

    let player_glyph = Glyph {
        glyph: '@'
    };

    let enemy_pos = WorldPosition {
        x: 10,
        y: 10,
        map: 0
    };

    let enemy_glyph = Glyph {
        glyph: 'M'
    };

    let enemy_health = Health {
        current_health: 10,
        max_health: 10,
    };

    let enemy_health_box = TextBox {
        value: String::from_str("?/?")?
    };

    let enemy_health_monitor = Monitor {
        to_monitor: vec![(String::from_str("enemy")?, String::from_str("Health")?)]
    };

    let enemy_health_pos = ScreenPosition {
        x: 0,
        y: 0
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
    game.add_component_from_data(&enemy_health, "enemy");
    game.add_component_from_data(&enemy_health_box, "enemy_hb");
    game.add_component_from_data(&enemy_health_monitor, "enemy_hb");
    game.add_component_from_data(&enemy_health_pos, "enemy_hb");

    let mut eq = GameEventQueue::new();

    let input_listener = Listener::new(
        vec!["input.key_press"], 
        "player", 
        player_move
    );

    let hit_listener = Listener::new(
        vec!["game.on_hit"],
        "enemy",
        on_hit
    );

    let update_listener = Listener::new(
        vec!["game.tick"],
        "enemy_hb",
        update_health
    );
    
    eq.attach_listener(input_listener);
    eq.attach_listener(hit_listener);
    eq.attach_listener(update_listener);

    run(&mut terminal, &mut game, &mut eq).context("app loop failed")?;
    rterm::restore_terminal(&mut terminal).context("restore terminal failed")?;

    Ok(())
}

// Render and poll terminal for keypress events
pub fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>, game : &mut GameManager, eq : &mut GameEventQueue) -> Result<()> {
    let mut cur_tick: u16 = 0;
    let start_ev = GameEvent {
        ev_type: "game.start".to_string(),
        data: "".to_string()
    };
    eq.trigger_listeners(game, start_ev);
 
    loop {
        terminal.draw(rterm::assemble_render(game))?;
        let key = rterm::poll()?;
        let input_ev = GameEvent {
            ev_type: "input.key_press".to_string(),
            data: serde_json::to_string( &InputData {
                key_code: key
            })?
        };

        let update_ev = GameEvent {
            ev_type: "game.tick".to_string(),
            data: serde_json::to_string( &TickData {
                tick: cur_tick
            } )?
        };

        eq.trigger_listeners(game, update_ev);

        if key == KeyCode::Esc { break }
        eq.trigger_listeners(game, input_ev);
        
    }
    Ok(())
}