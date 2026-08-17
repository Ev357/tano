use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::ListItem,
};

use crate::utils::layout_cache::get_list_area;

const SCROLL_OFFSET: usize = 5;

#[derive(Debug, Clone)]
pub struct ListState<T> {
    pub selected_index: Option<usize>,
    pub items: Vec<T>,
    pub offset: usize,
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
            offset: 0,
        }
    }

    fn align_offset(&mut self) {
        let height = get_list_area().height as usize;

        if height == 0 || self.items.is_empty() {
            return;
        }

        let selected = self.selected_index.unwrap_or(0);
        let effective_margin = SCROLL_OFFSET.min(height.saturating_sub(1) / 2);

        if selected < self.offset + effective_margin {
            self.offset = selected.saturating_sub(effective_margin);
        } else if selected + effective_margin >= self.offset + height {
            self.offset = (selected + effective_margin + 1).saturating_sub(height);
        }

        let max_offset = self.items.len().saturating_sub(height);
        self.offset = self.offset.min(max_offset);
    }

    pub fn next(&mut self) {
        if self.items.is_empty() {
            return;
        }

        let selected_index = self.selected_index.unwrap_or(0);
        self.selected_index = Some((selected_index + 1) % self.items.len());

        self.align_offset();
    }

    pub fn previous(&mut self) {
        if self.items.is_empty() {
            return;
        }

        let selected_index = self.selected_index.unwrap_or(self.items.len());

        if selected_index == 0 {
            self.selected_index = Some(self.items.len() - 1);
        } else {
            self.selected_index = Some(selected_index - 1);
        }

        self.align_offset();
    }

    pub fn jump_top(&mut self) {
        if self.items.is_empty() {
            return;
        }

        self.selected_index = Some(0);
        self.align_offset();
    }

    pub fn jump_bottom(&mut self) {
        if self.items.is_empty() {
            return;
        }
        self.selected_index = Some(self.items.len() - 1);
        self.align_offset();
    }

    pub fn scroll_up(&mut self) {
        if self.items.is_empty() {
            return;
        }

        let selected = self.selected_index.unwrap_or(0);
        self.selected_index = Some(selected.saturating_sub(1));
        self.align_offset();
    }

    pub fn scroll_down(&mut self) {
        if self.items.is_empty() {
            return;
        }

        let selected = self.selected_index.unwrap_or(0);
        self.selected_index = Some((selected + 1).min(self.items.len().saturating_sub(1)));
        self.align_offset();
    }

    pub fn scroll_percent(&mut self, percent: i32) {
        let height = get_list_area().height as i32;
        if height == 0 || self.items.is_empty() {
            return;
        }

        let jump_amount = (height * percent) / 100;
        let current_index = self.selected_index.unwrap_or(0) as isize;
        let new_index =
            (current_index + jump_amount as isize).clamp(0, (self.items.len() - 1) as isize);

        self.selected_index = Some(new_index as usize);

        self.align_offset();
    }

    pub fn selected(&self) -> Option<&T> {
        Some(&self.items[self.selected_index?])
    }

    pub fn displayed(&self) -> impl Iterator<Item = (bool, &T)> + '_ {
        let height = get_list_area().height as usize;

        let max_offset = self.items.len().saturating_sub(height);
        let start = self.offset.min(max_offset);
        let end = (start + height).min(self.items.len());

        let selected_idx = self.selected_index;

        self.items[start..end]
            .iter()
            .enumerate()
            .map(move |(index, item)| {
                let actual_index = start + index;
                let is_selected = selected_idx == Some(actual_index);

                (is_selected, item)
            })
    }

    pub fn to_list_items<F>(&self, item_to_text: F) -> Vec<ListItem<'_>>
    where
        F: Fn(&T) -> String,
    {
        let width = get_list_area().width as usize;
        let highlight_color = Color::Rgb(137, 180, 250);
        let text_color = Color::Rgb(30, 30, 46);

        self.displayed()
            .map(|(is_selected, item)| {
                let text = item_to_text(item);
                if !is_selected {
                    return ListItem::new(format!("  {}", text));
                }

                let text = format!(" {} ", text);
                let padding_len = width.saturating_sub(text.chars().count() + 2);
                let padded_text = format!("{}{}", text, " ".repeat(padding_len));

                let line = Line::from(vec![
                    Span::styled("", Style::default().fg(highlight_color)),
                    Span::styled(
                        padded_text,
                        Style::default()
                            .bg(highlight_color)
                            .fg(text_color)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled("", Style::default().fg(highlight_color)),
                ]);

                ListItem::new(line)
            })
            .collect()
    }
}
