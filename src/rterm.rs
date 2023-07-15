use std::{
    io::{self, Stdout},
    time::Duration, collections::HashMap, hash::Hash, ptr,
};

use anyhow::{Context, Result};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend, 
    widgets::Paragraph, 
    Terminal, 
    Frame,
    layout::Rect,
    text::{Line, Span}
};

use crate::game::{TileMap, TileType, GameEventQueue, GameManager, Glyph, WorldPosition};

/// Setup the terminal. This is where you would enable raw mode, enter the alternate screen, and
/// hide the cursor. This example does not handle errors. A more robust application would probably
/// want to handle errors and ensure that the terminal is restored to a sane state before exiting.
pub fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    let mut stdout = io::stdout();
    enable_raw_mode().context("failed to enable raw mode")?;
    execute!(stdout, EnterAlternateScreen).context("unable to enter alternate screen")?;
    Terminal::new(CrosstermBackend::new(stdout)).context("creating terminal failed")
}

/// Restore the terminal. This is where you disable raw mode, leave the alternate screen, and show
/// the cursor.
pub fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    disable_raw_mode().context("failed to disable raw mode")?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)
        .context("unable to switch to main screen")?;
    terminal.show_cursor().context("unable to show cursor")
}

pub fn assemble_render(game : &mut GameManager) -> Box<dyn FnMut(&mut Frame<CrosstermBackend<Stdout>>)> {
    // let objs : HashMap<String, GameObject> = objects.clone();
    // let map : TileMap = map.clone();

    let mut glyphs = game.get_components_by_type_mut("Glyph").unwrap();
    let mut glyphy: Vec<Glyph> = vec![];
    
    let maps = game.get_components_by_type_mut("TileMap").unwrap();
    let map: TileMap = serde_json::from_str(&maps[0].data.as_str()).unwrap();
    
    let closure = move |frame : &mut Frame<CrosstermBackend<Stdout>>| {

        let map_size = map.get_size();
        let mut text = vec![];
         
        for y in 0..map_size.1 {
            let mut line = "".to_string(); 
            for x in 0..map_size.0 {
                let glyph = map.tile_at((x,y));

                let ch = match glyph {
                    TileType::FLOOR => '.',
                    TileType::WALL => '█'
                };

                line.push(ch);
            }
            text.push(Line::from(line));
        }
        let grid = Paragraph::new(text);
        frame.render_widget(grid, frame.size());

        for glyph in glyphs.iter_mut() {

            let pos = game.get_components_by_obj_and_type_mut("Position", &glyph.obj_id).unwrap()[0];
            let pos: WorldPosition = serde_json::from_str(pos.data.as_str()).unwrap();

            let glyph_data: Glyph = serde_json::from_str(glyph.data.as_str()).unwrap();

            let render_at = Rect {
                x: pos.x,
                y: pos.y,
                width: 1,
                height: 1
            };

            let paragraph = Paragraph::new(glyph_data.glyph.to_string());

            frame.render_widget(paragraph, render_at);
        }
    };
    
    Box::new(closure)
}

/// Render the application. This is where you would draw the application UI. This example just
/// draws a greeting.
// pub fn render_app(frame: &mut ratatui::Frame<CrosstermBackend<Stdout>>) {
//     frame.render_widget(greeting, frame.size());
// }

/// Check if the user has pressed 'q'. This is where you would handle events. This example just
/// checks if the user has pressed 'q' and returns true if they have. It does not handle any other
/// events. There is a 250ms timeout on the event poll so that the application can exit in a timely
/// manner, and to ensure that the terminal is rendered at least once every 250ms.
pub fn poll() -> Result<KeyCode> {
    if event::poll(Duration::from_millis(250)).context("event poll failed")? {
        if let Event::Key(key) = event::read().context("event read failed")? {
            return Ok(key.code);
        }
    }
    Ok(KeyCode::Null)
}