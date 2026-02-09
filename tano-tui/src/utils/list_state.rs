#[derive(Debug, PartialEq, Clone)]
pub struct ListState<T> {
    pub selected_offset: Option<usize>,
    pub items: Vec<T>,
}

impl<T> ListState<T> {
    pub fn next(&mut self) {
        if self.items.is_empty() {
            return;
        }

        let selected_offset = match self.selected_offset {
            Some(offset) => offset,
            None => {
                self.selected_offset = Some(0);
                return;
            }
        };

        self.selected_offset = Some((selected_offset + 1) % self.items.len());
    }

    pub fn previous(&mut self) {
        if self.items.is_empty() {
            return;
        }

        let selected_offset = match self.selected_offset {
            Some(offset) => offset,
            None => {
                self.selected_offset = Some(self.items.len() - 1);
                return;
            }
        };

        if selected_offset == 0 {
            self.selected_offset = Some(self.items.len() - 1);
        } else {
            self.selected_offset = Some(selected_offset - 1);
        }
    }
}
