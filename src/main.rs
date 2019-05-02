// TODO find way to get rid of image
extern crate cgmath;
extern crate image;
extern crate piston_window; // used for pixel_buffer and Texture

use piston_window::*;

//use rand::Rng;
type Uint = u32;
type Vec2 = cgmath::Vector2<Uint>;

////////////////////////////
// (x,y)----width----->
//   |
//   h
//   e
//   i
//   g
//   h
//   t
//   |
//   |
//   v
struct Region {
    position: Vec2,
    width: Uint,
    height: Uint,
}

struct PlayerState {
    position: Vec2,
}

struct GameState {
    player_state: PlayerState,
    play_field: Region,
}

const WINDOW_SCALE: Uint = 3;
const PIXELS_PER_TILE: Uint = 8;

fn main() {
    let header = Region {
        position: Vec2 { x: 0, y: 0 },
        width: 28,
        height: 3,
    };
    let play_field = Region {
        position: Vec2 {
            x: 0,
            y: header.height,
        },
        width: 28,
        height: 31,
    };
    let footer = Region {
        position: Vec2 {
            x: 0,
            y: header.height + play_field.height,
        },
        width: 28,
        height: 2,
    };

    let (width, height) = (
        play_field.width * PIXELS_PER_TILE,
        (play_field.height + header.height + footer.height) * PIXELS_PER_TILE,
    );

    let mut window: PistonWindow = WindowSettings::new(
        "p a c m a n",
        ((width * WINDOW_SCALE), (height * WINDOW_SCALE)),
    )
    .exit_on_esc(true)
    .opengl(OpenGL::V3_2)
    .resizable(false)
    .decorated(false)
    .build()
    .unwrap();

    // texture for the pixel buffer
    let /*mut*/ pixel_buffer = image::ImageBuffer::new(width, height);
    let mut texture: G2dTexture = Texture::from_image(
        &mut window.factory,
        &pixel_buffer,
        &TextureSettings::new()
            .min(Filter::Linear)
            .mag(Filter::Linear)
            .mipmap(Filter::Nearest),
    )
    .unwrap();

    // INITALIZE
    let player_state = PlayerState {
        position: Vec2::new(0, 0),
    };
    let mut game_state = GameState {
        player_state: player_state,
        play_field: play_field,
    };

    while let Some(e) = window.next() {
        // UPDATE CONTROLLER
        let input = if let Some(piston_button_event) = e.button_args() {
            parse_piston_input_event(piston_button_event)
        } else {
            None
        };

        // UPDATE MODEL
        game_state = simulation_step(game_state, &input);

        // UPDATE VIEW
        // update pixel buffers
        let pacman_yellow = [1.0, 1.0, 0.0, 1.0];

        if let Some(_) = e.render_args() {
            texture.update(&mut window.encoder, &pixel_buffer).unwrap();
            window.draw_2d(&e, |c, g| {
                let clear_color = [0.2, 0.2, 0.2, 1.0];
                clear(clear_color, g);
                rectangle(
                    pacman_yellow,
                    [
                        (game_state.player_state.position.x * PIXELS_PER_TILE) as f64,
                        (game_state.player_state.position.y * PIXELS_PER_TILE) as f64,
                        PIXELS_PER_TILE as f64,
                        PIXELS_PER_TILE as f64,
                    ],
                    c.transform
                        .trans(
                            (game_state.play_field.position.x * PIXELS_PER_TILE) as f64,
                            (game_state.play_field.position.y * PIXELS_PER_TILE) as f64,
                        )
                        .scale(WINDOW_SCALE as f64, WINDOW_SCALE as f64),
                    g,
                );
            });
        }
    }
}

fn simulation_step(mut game_state: GameState, input: &Option<Input>) -> GameState {
    let Vec2 { x, y } = game_state.player_state.position;

    // Lesson: Trade off of disallowing invalid state to never entering invalid state
    let can_move_left = x != 0;
    let can_move_right = x != game_state.play_field.width - 1;
    let can_move_up = y != 0;
    let can_move_down = y != game_state.play_field.height - 1;

    match input {
        Some(Input::Down) if can_move_down => game_state.player_state.position.y += 1,

        Some(Input::Up) if can_move_up => game_state.player_state.position.y -= 1,

        Some(Input::Right) if can_move_right => game_state.player_state.position.x += 1,

        Some(Input::Left) if can_move_left => game_state.player_state.position.x -= 1,

        Some(Input::Down) | Some(Input::Up) | Some(Input::Right) | Some(Input::Left) | None => (),
    };
    game_state
}

////////////////////////////////////////////////////////////////////////////////
enum Input {
    Down,
    Up,
    Left,
    Right,
}

fn parse_piston_input_event(button_args: piston_window::ButtonArgs) -> Option<Input> {
    match button_args.state {
        ButtonState::Press => match button_args.button {
            Button::Keyboard(keyboard::Key::Up) => Some(Input::Up),
            Button::Keyboard(keyboard::Key::Down) => Some(Input::Down),
            Button::Keyboard(keyboard::Key::Left) => Some(Input::Left),
            Button::Keyboard(keyboard::Key::Right) => Some(Input::Right),
            _ => {
                println!("No mapping for button {:?}", button_args);
                None
            }
        },
        ButtonState::Release => None,
    }
}

// let transform = c.transform.scale(WINDOW_SCALE as f64, WINDOW_SCALE as f64);
// image(&texture, transform, g);
//line(
//    [0.0, 1.0, 1.0, 1.0],
//    0.2,
//    [0.0, 0.0, 244.0, 288.0],
//    c.transform,
//    g,
//);
// ellipse(
//     [1.0, 0.0, 0.0, 1.0],
//     [52.0, 152.0, 38.0, 138.0], // x, y, width, height
//     c.transform,
//     g,
// );
