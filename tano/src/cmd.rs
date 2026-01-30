use std::pin::Pin;

use color_eyre::Report;

use crate::{msg::Msg, update::handles::Handles};

pub type CmdFuture = Pin<Box<dyn Future<Output = Msg> + Send>>;
pub type CmdAction = Box<dyn FnOnce(Handles) -> CmdFuture + Send>;

pub enum Cmd {
    None,
    Task(CmdAction),
    Error(Report),
    Close,
    Msg(Msg),
}

impl Cmd {
    pub fn task<F, Fut>(action: F) -> Self
    where
        F: FnOnce(Handles) -> Fut + Send + 'static,
        Fut: Future<Output = Msg> + Send + 'static,
    {
        let wrapped_action = move |handles| {
            let future = action(handles);
            Box::pin(future) as Pin<Box<dyn Future<Output = Msg> + Send>>
        };

        Cmd::Task(Box::new(wrapped_action))
    }
}
