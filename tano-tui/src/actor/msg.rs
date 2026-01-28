use color_eyre::eyre::Result;

#[derive(Debug)]
pub enum TuiMsg {
    RenderDone(Result<()>),
}
