use piston_window::*;

#[derive(PartialEq)]
pub enum GameInput {
    Down,
    Up,
    Left,
    Right,
    Step,
    Nil,
}

impl From<keyboard::Key> for GameInput {
    fn from(key: keyboard::Key) -> GameInput {
        return match key {
            keyboard::Key::Up => GameInput::Up,
            keyboard::Key::Left => GameInput::Left,
            keyboard::Key::Down => GameInput::Down,
            keyboard::Key::Right => GameInput::Right,
            keyboard::Key::Space => GameInput::Step,
            _ => GameInput::Nil,
        }
    }
}

// There has to be an easier or nicer way to go
// Keyboard -> Input, Gamepad -> Input, w/ out having to deal with these different types
impl From<piston_window::Input> for GameInput {
    fn from(piston_input: piston_window::Input) -> GameInput {
        // if the input event was a button event
        if let piston_window::Input::Button(button_args) = piston_input {
            // if there was a button press
            if button_args.state == ButtonState::Press {
                // if is a keyboard button
                if let Button::Keyboard(keyboard_button) = button_args.button {
                    return GameInput::from(keyboard_button);
                }
            }
        }
        GameInput::Nil
    }
}
