use tano_tui::{utils::list_state::ListState, view::View};
use tokio::sync::watch::Sender;

use crate::{
    cmd::Cmd,
    model::{Model, config_state::ConfigState},
    msg::Msg,
    update::tui::TuiMsg,
};

pub fn update_refresh_view(model_tx: &Sender<Model>) -> Cmd {
    let pages = match &model_tx.borrow().config {
        ConfigState::Loaded { pages, .. } => pages.clone(),
        _ => return Cmd::None,
    };

    let modified = model_tx.send_if_modified(|model| match &mut model.view {
        View::Overview(props) => {
            let selected = props.sections.selected_index.unwrap_or(0);
            props.sections = ListState::new(pages.overview.sections, selected);

            true
        }
        View::Album(props) => {
            if props.config == pages.album {
                return false;
            }

            props.config = pages.album.clone();
            true
        }
        View::Loading | View::Song(_) | View::Songs(_) | View::Artists(_) | View::Albums(_) => {
            false
        }
    });

    if !modified {
        return Cmd::None;
    }

    Cmd::task(|handles| async move {
        let result = handles.tui.render().await;
        Msg::Tui(TuiMsg::RenderDone(result))
    })
}
