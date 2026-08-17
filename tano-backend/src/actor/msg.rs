use std::io;

use color_eyre::eyre::{Report, Result};
use crossterm::event::Event;

#[derive(Debug)]
pub enum BackendMsg {
    Event(Result<Event, io::Error>),
    Error(Report),
    Suspend,
    Resume,
}
