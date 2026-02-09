use crossterm::event::{Event, KeyCode};
use tano_backend::actor::msg::BackendMsg;
use tano_tui::{actor::msg::TuiMsg, view::View};
use tokio::sync::watch;

use crate::{cmd::Cmd, model::Model, msg::Msg};

pub fn update_backend(model_tx: &watch::Sender<Model>, backend_msg: BackendMsg) -> Cmd {
    match backend_msg {
        BackendMsg::Event(event) => match event {
            Ok(event) => match event {
                Event::Key(key_event) => match key_event.code {
                    KeyCode::Char('q') => Cmd::Msg(Msg::Restore),
                    KeyCode::Char(char @ ('j' | 'k')) => {
                        model_tx.send_modify(|model| {
                            let songs_props = match &mut model.view {
                                View::Songs(songs_props) => songs_props,
                                _ => return,
                            };

                            if char == 'j' {
                                songs_props.songs.next();
                            } else {
                                songs_props.songs.previous();
                            }
                        });

                        Cmd::task(|handles| async move {
                            let result = handles.tui.render().await;

                            Msg::Tui(TuiMsg::RenderDone(result))
                        })
                    }
                    _ => Cmd::None,
                },
                Event::Resize(_, _) => Cmd::task(|handles| async move {
                    let result = handles.tui.render().await;
                    Msg::Tui(TuiMsg::RenderDone(result))
                }),
                _ => Cmd::None,
            },
            Err(error) => Cmd::Error(error.into()),
        },
    }
}
