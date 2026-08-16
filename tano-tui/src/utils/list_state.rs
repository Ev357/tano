use std::sync::atomic::{AtomicUsize, Ordering};

const SCROLL_OFFSET: usize = 5;

#[derive(Debug)]
pub struct ListState<T> {
    pub selected_index: Option<usize>,
    pub items: Vec<T>,
    pub offset: AtomicUsize,
}

impl<T> ListState<T> {
    pub fn new(items: Vec<T>, selected_index: usize) -> Self {
        let selected_index = if selected_index < items.len() {
            Some(selected_index)
        } else {
            None
        };

        Self {
            selected_index,
            items,
            offset: AtomicUsize::new(0),
        }
    }

    pub fn next(&mut self) {
        if self.items.is_empty() {
            return;
        }

        let selected_index = match self.selected_index {
            Some(index) => index,
            None => {
                self.selected_index = Some(0);
                return;
            }
        };

        self.selected_index = Some((selected_index + 1) % self.items.len());
    }

    pub fn previous(&mut self) {
        if self.items.is_empty() {
            return;
        }

        let selected_index = match self.selected_index {
            Some(index) => index,
            None => {
                self.selected_index = Some(self.items.len() - 1);
                return;
            }
        };

        if selected_index == 0 {
            self.selected_index = Some(self.items.len() - 1);
        } else {
            self.selected_index = Some(selected_index - 1);
        }
    }

    pub fn jump_top(&mut self) {
        if self.items.is_empty() {
            return;
        }

        self.selected_index = Some(0);
    }

    pub fn jump_bottom(&mut self) {
        if self.items.is_empty() {
            return;
        }

        self.selected_index = Some(self.items.len() - 1);
    }

    pub fn selected(&self) -> Option<&T> {
        Some(&self.items[self.selected_index?])
    }

    pub fn displayed(&self, height: u16) -> impl Iterator<Item = (bool, &T)> + '_ {
        let height = height as usize;
        let mut current_offset = self.offset.load(Ordering::Relaxed);

        if height > 0 && !self.items.is_empty() {
            let selected = self.selected_index.unwrap_or(0);
            let effective_margin = SCROLL_OFFSET.min(height.saturating_sub(1) / 2);

            if selected < current_offset + effective_margin {
                current_offset = selected.saturating_sub(effective_margin);
            } else if selected + effective_margin >= current_offset + height {
                current_offset = (selected + effective_margin + 1).saturating_sub(height);
            }

            let max_offset = self.items.len().saturating_sub(height);
            current_offset = current_offset.min(max_offset);

            self.offset.store(current_offset, Ordering::Relaxed);
        }

        let start = current_offset;
        let end = (start + height).min(self.items.len());

        let selected_idx = self.selected_index;

        self.items[start..end]
            .iter()
            .enumerate()
            .map(move |(index, item)| {
                let actual_index = current_offset + index;
                let is_selected = selected_idx == Some(actual_index);

                (is_selected, item)
            })
    }
}

impl<T: Clone> Clone for ListState<T> {
    fn clone(&self) -> Self {
        Self {
            items: self.items.clone(),
            selected_index: self.selected_index,
            offset: AtomicUsize::new(self.offset.load(Ordering::Relaxed)),
        }
    }
}
