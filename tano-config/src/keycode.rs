use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum KeyCode {
    Char(char),
    Backspace,
    Enter,
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
    Tab,
    BackTab,
    Delete,
    Insert,
    F(u8),
    Esc,
}

impl FromStr for KeyCode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "space" => Ok(KeyCode::Char(' ')),
            "backspace" => Ok(KeyCode::Backspace),
            "enter" => Ok(KeyCode::Enter),
            "left" => Ok(KeyCode::Left),
            "right" => Ok(KeyCode::Right),
            "up" => Ok(KeyCode::Up),
            "down" => Ok(KeyCode::Down),
            "home" => Ok(KeyCode::Home),
            "end" => Ok(KeyCode::End),
            "pageup" => Ok(KeyCode::PageUp),
            "pagedown" => Ok(KeyCode::PageDown),
            "tab" => Ok(KeyCode::Tab),
            "backtab" => Ok(KeyCode::BackTab),
            "delete" => Ok(KeyCode::Delete),
            "insert" => Ok(KeyCode::Insert),
            "esc" => Ok(KeyCode::Esc),
            other => {
                if other.starts_with('f')
                    && let Ok(num) = other[1..].parse::<u8>()
                {
                    return Ok(KeyCode::F(num));
                }

                let original_chars: Vec<_> = s.chars().collect();
                if original_chars.len() != 1 {
                    return Err(format!("Unknown key: {}", s));
                }

                Ok(KeyCode::Char(original_chars[0]))
            }
        }
    }
}
