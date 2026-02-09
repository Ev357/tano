#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct WatchMode {
    pub create: bool,
    pub modify: bool,
    pub remove: bool,
}

impl WatchMode {
    pub fn all() -> Self {
        Self {
            create: true,
            modify: true,
            remove: true,
        }
    }

    pub fn create(mut self) -> Self {
        self.create = true;
        self
    }

    pub fn modify(mut self) -> Self {
        self.modify = true;
        self
    }

    pub fn remove(mut self) -> Self {
        self.remove = true;
        self
    }
}
