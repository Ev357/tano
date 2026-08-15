use color_eyre::eyre::Result;
use tano_database::song::Song;
use tano_tui::{
    components::songs::SongsProps,
    utils::{list_state::ListState, load_state::LoadState},
    view::View,
};
use tokio::sync::watch;

use crate::{cmd::Cmd, model::Model};

#[derive(Debug)]
pub enum DatabaseMsg {
    SongsLoaded { songs: Result<Vec<Song>> },
}

pub fn update_database(model_tx: &watch::Sender<Model>, database_msg: DatabaseMsg) -> Cmd {
    match database_msg {
        DatabaseMsg::SongsLoaded { songs } => match songs {
            Ok(songs) => {
                let songs = ListState::new(songs, 0);

                model_tx.send_modify(|model| {
                    model.view = View::Songs(SongsProps {
                        songs: LoadState::Loaded(songs),
                    })
                });

                Cmd::None
            }
            Err(report) => Cmd::Error(report),
        },
    }
}
