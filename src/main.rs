use anyhow::{Context, Result};
use ratatui::{backend::CrosstermBackend, widgets::Paragraph, Terminal};
use std::{
    io::{self, Stdout},
    time::Duration,
};

mod rterm;

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
    run(&mut terminal).context("app loop failed")?;
    rterm::restore_terminal(&mut terminal).context("restore terminal failed")?;
    Ok(())
}

/// Run the application loop. This is where you would handle events and update the application
/// state. This example exits when the user presses 'q'. Other styles of application loops are
/// possible, for example, you could have multiple application states and switch between them based
/// on events, or you could have a single application state and update it based on events.
pub fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    loop {
        terminal.draw(rterm::render_app)?;
        if rterm::should_quit()? {
            break;
        }
    }
    Ok(())
}