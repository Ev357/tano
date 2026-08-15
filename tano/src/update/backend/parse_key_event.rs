use crossterm::event::{KeyCode as CrossKeyCode, KeyEvent, KeyModifiers as CrossModifiers};
use tano_config::keymaps::{key_modifiers::KeyModifiers, keybind::KeyBind, keycode::KeyCode};

pub fn parse_key_event(event: &KeyEvent) -> Option<KeyBind> {
    let mut modifiers = KeyModifiers::empty();

    if event.modifiers.contains(CrossModifiers::CONTROL) {
        modifiers.insert(KeyModifiers::CONTROL);
    }
    if event.modifiers.contains(CrossModifiers::SHIFT) {
        modifiers.insert(KeyModifiers::SHIFT);
    }
    if event.modifiers.contains(CrossModifiers::ALT) {
        modifiers.insert(KeyModifiers::ALT);
    }
    if event.modifiers.contains(CrossModifiers::SUPER) {
        modifiers.insert(KeyModifiers::SUPER);
    }
    if event.modifiers.contains(CrossModifiers::HYPER) {
        modifiers.insert(KeyModifiers::HYPER);
    }
    if event.modifiers.contains(CrossModifiers::META) {
        modifiers.insert(KeyModifiers::META);
    }

    let code = match event.code {
        CrossKeyCode::Char(c) => KeyCode::Char(c.to_ascii_lowercase()),
        CrossKeyCode::Backspace => KeyCode::Backspace,
        CrossKeyCode::Enter => KeyCode::Enter,
        CrossKeyCode::Left => KeyCode::Left,
        CrossKeyCode::Right => KeyCode::Right,
        CrossKeyCode::Up => KeyCode::Up,
        CrossKeyCode::Down => KeyCode::Down,
        CrossKeyCode::Home => KeyCode::Home,
        CrossKeyCode::End => KeyCode::End,
        CrossKeyCode::PageUp => KeyCode::PageUp,
        CrossKeyCode::PageDown => KeyCode::PageDown,
        CrossKeyCode::Tab => KeyCode::Tab,
        CrossKeyCode::BackTab => KeyCode::BackTab,
        CrossKeyCode::Delete => KeyCode::Delete,
        CrossKeyCode::Insert => KeyCode::Insert,
        CrossKeyCode::F(n) => KeyCode::F(n),
        CrossKeyCode::Esc => KeyCode::Esc,
        _ => return None,
    };

    Some(KeyBind { code, modifiers })
}
