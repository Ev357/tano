use std::{ops::Deref, slice};

use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(untagged)]
pub enum OneOrMany<T> {
    One(T),
    Many(Vec<T>),
}

impl<T> Deref for OneOrMany<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        match self {
            Self::One(val) => slice::from_ref(val),
            Self::Many(vec) => vec.as_slice(),
        }
    }
}
