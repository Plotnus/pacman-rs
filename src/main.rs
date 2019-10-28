extern crate cgmath;
extern crate piston_window;

use piston_window::*;

mod board;
use crate::board::*;

type Vec2 = cgmath::Vector2<f32>;

struct CharacterState {
    position: Vec2,
    move_dir: Vec2,
}

struct GameState {
    player: CharacterState,
    board: Board,
    ready_to_process_turn: bool,
}

const PIXELS_PER_TILE: usize = 8;
const WINDOW_SCALE: usize = 3;

fn main() {
    let board = Board::new();

    let num_px_wide = board.width * PIXELS_PER_TILE * WINDOW_SCALE;
    let num_px_high = board.height * PIXELS_PER_TILE * WINDOW_SCALE;
    let window_size = (num_px_wide as u32, num_px_high as u32);

    let mut window: PistonWindow = WindowSettings::new("p a c m a n", window_size)
        .exit_on_esc(true)
        .graphics_api(Api::opengl(3, 2))
        .resizable(false)
        .decorated(false)
        .build()
        .unwrap();

    let x = (board.width / 2 * PIXELS_PER_TILE) as f32;
    let y = ((board.height / 2 + 5) * PIXELS_PER_TILE) as f32 + PIXELS_PER_TILE as f32 * 0.5;
    let player = CharacterState {
        position: Vec2::new(x,y),
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
        }

        // UPDATE VIEW
        if let Some(_) = e.render_args() {
            //          texture.update(&mut window.encoder, &pixel_buffer).unwrap();
            window.draw_2d(&e, |context, g, _| {
                let clear_color = [0.0, 0.0, 0.0, 1.0];
                clear(clear_color, g);

                // draw board
                let board = &gamestate.board;
                let transform = context
                    .transform
                    .scale(WINDOW_SCALE as f64, WINDOW_SCALE as f64);

                // draw walls
                for h in 0..board.num_tiles {
                    if !board.tile_is_traversable(h) {
                        let pos = board.get_board_pos_of_tile(h);
                        const WALL_COLOR: [f32;4] = [0.0, 0.0, 0.6, 1.0 ];
                        draw_tile(pos, WALL_COLOR, transform, g);
                    }
                }

                // draw tunnels
                for h in 0..board.num_tiles {
                    if board.tile_is_tunnel(h) {
                        const TUNNEL_COLOR: [f32;4] = [0.2, 0.2, 0.2, 1.0];
                        let pos = board.get_board_pos_of_tile(h);
                        draw_tile(pos, TUNNEL_COLOR, transform, g);
                    }
                }

                // draw pellets
                for h in 0..board.num_tiles {
                    if board.tile_has_pellet(h) {
                        const PELLET_COLOR: [f32; 4] = [0.8,0.8,0.8,1.0];
                        const SCALE: f32 = 0.25;
                        let pos = board.get_board_pos_of_tile(h);
                        draw_circle(pos, PELLET_COLOR, SCALE, transform, g);
                    }
                }

                // draw power_pellets
                for h in 0..board.num_tiles {
                    if board.tile_has_power_pellet(h) {
                        const PELLET_COLOR: [f32; 4] = [0.8,0.8,0.8,1.0];
                        const SCALE: f32 = 0.5;
                        let pos = board.get_board_pos_of_tile(h);
                        draw_circle(pos, PELLET_COLOR, SCALE, transform, g);
                    }
                }

                { // draw grid
                    let grid = grid::Grid {
                        cols: gamestate.board.width as u32,
                        rows: gamestate.board.height as u32,
                        units: (PIXELS_PER_TILE * WINDOW_SCALE) as f64,
                    };
                    let line = Line {
                        color: [0.8, 0.8, 0.8, 1.0], // <--- grey
                        radius: 0.5,
                        shape: line::Shape::Round,
                    };
                    grid.draw(&line, &context.draw_state, context.transform, g);
                }

                { // draw player
                    let x = gamestate.player.position.x as f64;
                    let y = gamestate.player.position.y as f64;
                    let shift = PIXELS_PER_TILE as f64 * 0.5;
                    // body
                    let color = [1.0, 1.0, 0.0, 0.5];
                    let rect = [
                        x - shift,
                        y - shift,
                        PIXELS_PER_TILE as f64,
                        PIXELS_PER_TILE as f64,
                    ];
                    let transform = context
                        .transform
                        .scale(WINDOW_SCALE as f64, WINDOW_SCALE as f64);
                    // draw tile_pos as yellow tile
                    // draw pixel_pos as red dot
                    // draw black triangle an direction
                    rectangle(color, rect, transform, g);
                    // direction signifier
                    let line = Line {
                        color: [0.0, 0.0, 0.0, 1.0],
                        radius: 1.0,
                        shape: line::Shape::Round,
                    };
                    let a = [x + 1.0, y, x - 1.0, y - 1.0];
                    let b = [x - 1.0, y - 1.0, x - 1.0, y + 1.0];
                    let c = [x - 1.0, y + 1.0, x + 1.0, y];
                    let t = context
                        .transform
                        .scale(1.5 * WINDOW_SCALE as f64, 1.5 * WINDOW_SCALE as f64);
                    line.draw_tri(a, &context.draw_state, t, g);
                    line.draw_tri(b, &context.draw_state, t, g);
                    line.draw_tri(c, &context.draw_state, t, g);
                    // pixel position
                    let color = [1.0, 0.0, 0.0, 1.0];
                    let rect = [x, y, 1.0, 1.0];
                    rectangle(color, rect, transform, g);
                }
            });
        }
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


pub fn draw_tile<G>(
    pos: BoardPos,
    color: [f32;4],
    transform: math::Matrix2d,
    g: &mut G)
    where G: Graphics
{
    let w = PIXELS_PER_TILE as f64;
    let h = w;
    let x = pos.x as f64 * w;
    let y = pos.y as f64 * h;
    let rect = [ x, y, w, h, ];

    // draw tile
    rectangle(color, rect, transform, g);
}

pub fn draw_circle<G>(
    pos: BoardPos,
    color: [f32;4],
    scale: f32,
    transform: math::Matrix2d,
    g: &mut G)
    where G: Graphics
{
    let w = PIXELS_PER_TILE as f64 * scale as f64;
    let h = w;

    let x = (pos.x * PIXELS_PER_TILE) as f64
          + PIXELS_PER_TILE as f64 * 0.5
          - w * 0.5;
    let y = (pos.y * PIXELS_PER_TILE) as f64
          + PIXELS_PER_TILE as f64 * 0.5
          - h * 0.5;
    let rect: [f64;4] = [
        x as f64,
        y as f64,
        w as f64,
        h as f64,
    ];
    Ellipse::new(color).draw(rect, &Default::default(), transform, g);
}
