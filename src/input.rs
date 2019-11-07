use piston_window::*;

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
