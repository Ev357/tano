use tano_config::{action::Action, keybind::KeyBind};

use crate::model::Model;

pub fn handle_keypress(model: &mut Model, key: KeyBind) -> Option<Action> {
    model.keybind_buffer.push(key);

    let mut current_node = &model.keymap;

    for buffered_key in &model.keybind_buffer {
        match current_node.children.get(buffered_key) {
            Some(next_node) => current_node = next_node,
            None => {
                let was_in_sequence = model.keybind_buffer.len() > 1;

                model.keybind_buffer.clear();

                if !was_in_sequence {
                    return None;
                }

                return handle_keypress(model, key);
            }
        }
    }

    let action = current_node.action?;

    model.keybind_buffer.clear();

    Some(action)
}
