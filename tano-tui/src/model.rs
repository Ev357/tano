use crate::view::View;

pub trait TuiModel: Send + Sync + 'static {
    fn view(&self) -> &View;
}
