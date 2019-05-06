// TODO find way to get rid of image
extern crate cgmath;
extern crate image;
extern crate piston_window; // used for pixel_buffer and Texture

use piston_window::*;

type Vec2 = cgmath::Vector2<f32>;

////////////////////////////////////////////////////////////////////////////////////
// COLORS
//use rand::Rng;
struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

struct Rgb24 {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    fn new() -> Color {
        Color {
            r: 0_f32,
            g: 0_f32,
            b: 0_f32,
            a: 1_f32,
        }
    }

    fn from_rgb24(rgb: Rgb24) -> Color {
        let scale = 1_f32 / std::u8::MAX as f32;
        Color {
            r: rgb.r as f32 * scale,
            g: rgb.g as f32 * scale,
            b: rgb.b as f32 * scale,
            a: 1_f32,
        }
    }

    fn to_vec4(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }
}
////////////////////////////////////////////////////////////////////////////////////
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

struct Tile {
    color: Color,
    has_pellet: bool,
    has_power_pellet: bool,
    is_traversable: bool,
}

////////////////////////////////////////////////////////////////////////////////
struct Board {
    tiles: Vec<Tile>,
    width: u32,
    height: u32,
}

impl Board {
    fn new(width: u32, height: u32) -> Board {
        let tiles: Vec<Tile> = Vec::new();
        Board {
            tiles,
            width,
            height,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
struct CharacterState {
    position: Vec2,
    move_dir: Vec2,
}

struct GameState {
    player: CharacterState,
    board: Board,
    ready_to_process_turn: bool,
}

const PIXELS_PER_TILE: u32 = 8;
const WINDOW_SCALE: u32 = 3;

fn main() {
    let board = Board::new(28, 31);

    let num_px_wide = board.width * PIXELS_PER_TILE * WINDOW_SCALE;
    let num_px_high = board.height * PIXELS_PER_TILE * WINDOW_SCALE;
    let window_size = (num_px_wide, num_px_high);

    let mut window: PistonWindow = WindowSettings::new("p a c m a n", window_size)
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
    let player = CharacterState {
        position: Vec2::new(0_f32, 0_f32),
        move_dir: Vec2::new(0_f32, 0_f32),
    };

    // INITALIZE
    let mut gamestate = GameState {
        player,
        board,
        ready_to_process_turn: false,
    };

    while let Some(e) = window.next() {
        // RESET FRAME VARS
        gamestate.ready_to_process_turn = false;

        // INPUT
        if let Some(piston_button_event) = e.button_args() {
            let optional_input = parse_piston_input_event(piston_button_event);
            if let Some(input) = optional_input {
                match input {
                    Input::Down => gamestate.player.move_dir = World::DOWN,
                    Input::Up => gamestate.player.move_dir = World::UP,
                    Input::Right => gamestate.player.move_dir = World::RIGHT,
                    Input::Left => gamestate.player.move_dir = World::LEFT,
                    Input::Step => gamestate.ready_to_process_turn = true,
                }
            }
        }

        // UPDATE MODEL
        if gamestate.ready_to_process_turn {
            // update position
            gamestate.player.position += gamestate.player.move_dir;

            // x-axis wrap
            let x_min = 0;
            let x_max = (gamestate.board.width * PIXELS_PER_TILE - 1) as i32;
            let x = gamestate.player.position.x as i32;
            if x < x_min {
                gamestate.player.position.x = x_max as f32;
            } else if x > x_max {
                gamestate.player.position.x = x_min as f32;
            }

            // y-axis wrap
            let y_min = 0;
            let y_max = (gamestate.board.height * PIXELS_PER_TILE - 1) as i32;
            let y = gamestate.player.position.y as i32;
            if y < y_min {
                gamestate.player.position.y = y_max as f32;
            } else if y > y_max {
                gamestate.player.position.y = y_min as f32;
            }

            // TODO collision
        }

        // UPDATE VIEW
        // update pixel buffers
        if let Some(_) = e.render_args() {
            //          texture.update(&mut window.encoder, &pixel_buffer).unwrap();
            window.draw_2d(&e, |c, g| {
                let clear_color = [0.2, 0.2, 0.2, 1.0];
                clear(clear_color, g);
                // draw tiles
                let grid = grid::Grid {cols: gamestate.board.width,
                    rows: gamestate.board.height,
                    units: (PIXELS_PER_TILE * WINDOW_SCALE) as f64,
                };
                let line = Line {
                    color: [0.8, 0.8, 0.8, 1.0], // <--- grey
                    radius: 0.5,
                    shape: line::Shape::Round,
                };
                grid.draw(&line, &c.draw_state, c.transform, g);
                // draw dots
                // draw fruits
                // draw score
                // draw pacman
                let color = [1.0, 1.0, 0.0, 0.5];
                let rect = [
                    gamestate.player.position.x as f64,
                    gamestate.player.position.y as f64,
                    PIXELS_PER_TILE as f64,
                    PIXELS_PER_TILE as f64,
                ];
                let transform = c
                    .transform
                    .trans(0.0, 0.0)
                    .scale(WINDOW_SCALE as f64, WINDOW_SCALE as f64);
                rectangle(color, rect, transform, g);
                gamestate.board.width;
                gamestate.board.height;

                // draw ghosts
                //
            });
        }

        // RESET FRAME VARIABLES
    }
}

struct World {}

////////////////////////////////////////////////////////////////////////////////
impl World {
    const LEFT: Vec2 = Vec2::new(-1_f32, 0_f32);
    const RIGHT: Vec2 = Vec2::new(1_f32, 0_f32);
    const UP: Vec2 = Vec2::new(0_f32, -1_f32);
    const DOWN: Vec2 = Vec2::new(0_f32, 1_f32);
}

////////////////////////////////////////////////////////////////////////////////
#[derive(PartialEq)]
enum Input {
    Down,
    Up,
    Left,
    Right,
    Step,
}

fn parse_piston_input_event(button_args: piston_window::ButtonArgs) -> Option<Input> {
    if button_args.state == ButtonState::Press {
        if let Button::Keyboard(key) = button_args.button {
            return match key {
                keyboard::Key::Up | keyboard::Key::Period => Some(Input::Up),
                keyboard::Key::Left | keyboard::Key::O => Some(Input::Left),
                keyboard::Key::Down | keyboard::Key::E => Some(Input::Down),
                keyboard::Key::Right | keyboard::Key::U => Some(Input::Right),
                keyboard::Key::Space => Some(Input::Step),
                _ => None,
            };
        }
    }
    None
}

// PixelPosition => TilePosition
// update move_direction based on controllers
// position += move_direction * move_speed;
// tile_position = position * (1/ NUM_PIXELS_PER_TILE);
