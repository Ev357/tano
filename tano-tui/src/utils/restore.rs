use std::io::stdout;

use color_eyre::eyre::Result;
use crossterm::{
    execute,
    terminal::{LeaveAlternateScreen, disable_raw_mode},
};

pub fn restore() -> Result<()> {
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;

    Ok(())
}
