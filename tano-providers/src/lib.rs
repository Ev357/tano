use crate::local::LocalProvider;

pub mod local;

#[derive(Debug)]
pub enum ProviderType {
    Local(LocalProvider),
}
