use std::cell::RefCell;

use ratatui::layout::Rect;

#[derive(Default, Clone, Copy)]
pub struct LayoutCache {
    pub list_area: Rect,
}

impl LayoutCache {
    pub fn new() -> Self {
        Self::default()
    }
}

thread_local! {
    static LAYOUT_CACHE: RefCell<LayoutCache> = RefCell::new(LayoutCache::new());
}

pub fn update_list_area(area: Rect) {
    LAYOUT_CACHE.with(|cache| {
        cache.borrow_mut().list_area = area;
    });
}

pub fn get_list_area() -> Rect {
    LAYOUT_CACHE.with(|cache| cache.borrow().list_area)
}
