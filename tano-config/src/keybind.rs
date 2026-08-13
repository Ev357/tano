use std::str::FromStr;

use serde::{Deserialize, Deserializer, de};

use crate::{key_modifiers::KeyModifiers, keycode::KeyCode};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct KeyBind {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
}

impl FromStr for KeyBind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = if s != " " { s.trim() } else { s };

        if s.is_empty() {
            return Err("Keybinding cannot be empty".to_string());
        }

        let mut modifiers = KeyModifiers::empty();
        let mut key_str = s;

        if s.starts_with('<') && s.ends_with('>') {
            let inner = &s[1..s.len() - 1];
            let mut parts: Vec<_> = inner.split('-').collect();
            key_str = parts.pop().ok_or("Empty brackets < >")?;

            for modifier in parts {
                match modifier {
                    "C" => modifiers.insert(KeyModifiers::CONTROL),
                    "S" => modifiers.insert(KeyModifiers::SHIFT),
                    "A" => modifiers.insert(KeyModifiers::ALT),
                    "D" => modifiers.insert(KeyModifiers::SUPER),
                    "H" => modifiers.insert(KeyModifiers::HYPER),
                    "M" => modifiers.insert(KeyModifiers::META),
                    _ => return Err(format!("Unknown modifier: {}", modifier)),
                }
            }
        }

        let mut code = if key_str.eq_ignore_ascii_case("space") || key_str == " " {
            KeyCode::Char(' ')
        } else {
            KeyCode::from_str(key_str)?
        };

        // Normalize Shift/Uppercase logic
        if let KeyCode::Char(c) = code {
            if c.is_uppercase() {
                modifiers.insert(KeyModifiers::SHIFT);
                code = KeyCode::Char(c.to_ascii_lowercase());
            }
        } else if let KeyCode::BackTab = code {
            modifiers.insert(KeyModifiers::SHIFT);
            code = KeyCode::Tab;
        }

        Ok(KeyBind { code, modifiers })
    }
}

impl<'de> Deserialize<'de> for KeyBind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        KeyBind::from_str(&s).map_err(de::Error::custom)
    }
}
