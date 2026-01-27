use std::io;

use color_eyre::eyre::Result;
use crossterm::event::Event;

#[derive(Debug)]
pub enum BackendMsg {
    Event(Result<Event, io::Error>),
}
