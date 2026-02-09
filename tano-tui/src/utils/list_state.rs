#[derive(Debug, PartialEq, Clone)]
pub struct ListState<T> {
    pub selected_index: Option<usize>,
    pub items: Vec<T>,
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
}
