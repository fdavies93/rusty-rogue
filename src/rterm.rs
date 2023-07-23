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
    widgets::{Paragraph}, 
    Terminal, 
    Frame,
    layout::Rect,
    text::{Line}
};

use crate::components::{Glyph, TileMap, TileType, WorldPosition};
use crate::game::GameManager;

pub fn clamp(val: u16, min: u16, max: u16) -> u16 {
    if val < min {
        return min;
    }
    if val > max {
        return max;
    }
    val
}

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

    let mut widgets: Vec<(Paragraph, Rect)> = vec![];

    let mut glyphs = {
        let mut glyphy: Vec<(String, Glyph)> = vec![];
        let glyph_comps = game.get_components_by_type_mut("Glyph").unwrap();
        for comp in glyph_comps {
            glyphy.push( (comp.obj_id.clone(), serde_json::from_str(comp.data.as_str()).unwrap()) )
        }
        glyphy
    };

    let mut glyph_positions = {
        let mut glyph_pos = vec![];
        for glyph in glyphs {
            let comp_option = &game.get_components("WorldPosition", &glyph.0);
            let comps = match (comp_option) {
                None => continue,
                Some(c) => c
            };
            if comps.len() == 0 { continue }
            let comp = &comps[0];
            let pos_data: WorldPosition = serde_json::from_str(comp.data.as_str()).unwrap();
            glyph_pos.push((pos_data, glyph.1));
        }
        glyph_pos
    };

    let map: TileMap = 
    {
        let maps = game.get_components_by_type_mut("TileMap").unwrap();
        serde_json::from_str(&maps[0].data.as_str()).unwrap()
    };

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

    // render map
    let grid = Paragraph::new(text);
    widgets.push((grid, Rect {
        x: 0,
        y: 0,
        width: map_size.0,
        height: map_size.1
    }));

    // render map objects
    for pos_glyph in glyph_positions {
        widgets.push((
            Paragraph::new(pos_glyph.1.glyph.to_string()),
            Rect::new(pos_glyph.0.x, pos_glyph.0.y, 1, 1)
        ))
    }

    let closure = move |frame : &mut Frame<CrosstermBackend<Stdout>>| {

        // add logic for outputting objects in correct place

        for widget in &mut widgets {
            let f = frame.size();
            let r_x = clamp(widget.1.x, 0, f.right());
            let r_y = clamp(widget.1.y, 0, f.bottom());
            let r = Rect{
                x: r_x,
                y: r_y,
                width: clamp(widget.1.width, 0, f.right() - r_x),
                height: clamp(widget.1.height, 0, f.bottom() - r_y)
            };

            frame.render_widget(widget.0.clone(),r);
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