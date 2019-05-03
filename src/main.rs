// TODO find way to get rid of image
extern crate cgmath;
extern crate image;
extern crate piston_window; // used for pixel_buffer and Texture

use piston_window::*;

//use rand::Rng;
type Vec2 = cgmath::Vector2<i32>;

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
    width: u16,
    height: u16,
}

struct CharacterState {
    position: Vec2,
    move_dir: Vec2,
}

struct GameState {
    player: CharacterState,
    playfield: Region,
}

const WINDOW_SCALE: u16 = 3;
const PIXELS_PER_TILE: u16 = 8;

fn main() {
    let playfield = Region {
        position: Vec2 { x: 0, y: 0 },
        width: 28,
        height: 31,
    };

    let (width, height) = (playfield.width, playfield.height);

    let mut window: PistonWindow = WindowSettings::new(
        "p a c m a n",
        (
            (width * PIXELS_PER_TILE * WINDOW_SCALE) as u32,
            (height * PIXELS_PER_TILE * WINDOW_SCALE) as u32,
        ),
    )
    .exit_on_esc(true)
    .opengl(OpenGL::V3_2)
    .resizable(false)
    .decorated(true)
    .build()
    .unwrap();

    // create texture for the pixel buffer
    //    let /*mut*/ pixel_buffer = image::ImageBuffer::new(width, height);
    //    let mut texture: G2dTexture = Texture::from_image(
    //        &mut window.factory,
    //        &pixel_buffer,
    //        &TextureSettings::new()
    //            .min(Filter::Linear)
    //            .mag(Filter::Linear)
    //            .mipmap(Filter::Nearest),
    //    )
    //    .unwrap();

    // INITALIZE
    let mut gamestate = GameState {
        player: CharacterState {
            position: Vec2::new(0, 0),
            move_dir: Vec2::new(0, 0),
        },
        playfield: playfield,
    };

    while let Some(e) = window.next() {
        // APPLY INPUT to GAME
        let input = if let Some(piston_button_event) = e.button_args() {
            parse_piston_input_event(piston_button_event)
        } else {
            Input::None
        };

        // UPDATE MODEL
        gamestate = step_game(gamestate, input);

        // UPDATE VIEW
        // update pixel buffers
        let pacman_yellow = [1.0, 1.0, 0.0, 1.0];

        if let Some(_) = e.render_args() {
            //          texture.update(&mut window.encoder, &pixel_buffer).unwrap();
            window.draw_2d(&e, |c, g| {
                let clear_color = [0.2, 0.2, 0.2, 1.0];
                clear(clear_color, g);

                rectangle(
                    pacman_yellow,
                    [
                        (gamestate.player.position.x * PIXELS_PER_TILE as i32) as f64,
                        (gamestate.player.position.y * PIXELS_PER_TILE as i32) as f64,
                        PIXELS_PER_TILE as f64,
                        PIXELS_PER_TILE as f64,
                    ],
                    c.transform
                        .trans(
                            (gamestate.playfield.position.x * PIXELS_PER_TILE as i32) as f64,
                            (gamestate.playfield.position.y * PIXELS_PER_TILE as i32) as f64,
                        )
                        .scale(WINDOW_SCALE as f64, WINDOW_SCALE as f64),
                    g,
                );
            });
        }
    }
}

struct World {}

impl World {
    const LEFT: Vec2 = Vec2::new(-1, 0);
    const RIGHT: Vec2 = Vec2::new(1, 0);
    const UP: Vec2 = Vec2::new(0, -1);
    const DOWN: Vec2 = Vec2::new(0, 1);
}

fn step_game(mut gamestate: GameState, input: Input) -> GameState {
    // STEP CONTROLLERS
    let requested_move_direction = match input {
        Input::Down => World::DOWN,
        Input::Up => World::UP,
        Input::Right => World::RIGHT,
        Input::Left => World::LEFT,
        _ => Vec2::new(0, 0),
    };

    let can_move_in_requested_direction = {
        let new_pos = gamestate.player.position + requested_move_direction;
        0 <= new_pos.x
            && new_pos.x < gamestate.playfield.width as i32
            && 0 <= new_pos.y
            && new_pos.y < gamestate.playfield.height as i32
    };

    if can_move_in_requested_direction {
        gamestate.player.move_dir = requested_move_direction;
    }

    // STEP SIMULATION
    gamestate.player.position += gamestate.player.move_dir;

    // RETURN NEXT STATE
    gamestate
}

////////////////////////////////////////////////////////////////////////////////
enum Input {
    Down,
    Up,
    Left,
    Right,
    None,
}

fn parse_piston_input_event(button_args: piston_window::ButtonArgs) -> Input {
    match button_args.state {
        ButtonState::Press => match button_args.button {
            Button::Keyboard(keyboard::Key::Up) => Input::Up,
            Button::Keyboard(keyboard::Key::Down) => Input::Down,
            Button::Keyboard(keyboard::Key::Left) => Input::Left,
            Button::Keyboard(keyboard::Key::Right) => Input::Right,
            _ => Input::None,
        },
        ButtonState::Release => Input::None,
    }
}

// PixelPosition => TilePosition
// update move_direction based on controllers
// position += move_direction * move_speed;
// tile_position = position * (1/ NUM_PIXELS_PER_TILE);
