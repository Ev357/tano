use std::pin::Pin;

use color_eyre::Report;

use crate::{handle_message::Handles, msg::Msg};

#[derive(Debug)]
pub enum Cmd {
    None,
    Some(fn(Handles) -> Pin<Box<dyn Future<Output = Msg> + Send>>),
    Error(Report),
    Close,
    Msg(Msg),
}
