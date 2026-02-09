use tano_database::actor::mgs::DatabaseMsg;
use tano_tui::{components::songs::SongsProps, utils::list_state::ListState, view::View};
use tokio::sync::watch;

use crate::{cmd::Cmd, model::Model};

pub fn update_database(model_tx: &watch::Sender<Model>, database_msg: DatabaseMsg) -> Cmd {
    match database_msg {
        DatabaseMsg::SongsLoaded { songs } => match songs {
            Ok(songs) => {
                let songs = ListState::new(songs, 0);

                model_tx.send_modify(|model| model.view = View::Songs(SongsProps { songs }));

                Cmd::None
            }
            Err(report) => Cmd::Error(report),
        },
    }
}
