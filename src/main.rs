// TODO find way to get rid of image
extern crate cgmath;
extern crate image;
extern crate piston_window;

use piston_window::*;

mod board;
use crate::board::*;

type Vec2 = cgmath::Vector2<f32>;

////////////////////////////////////////////////////////////////////////////////////
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
    let board = Board::new();

    let num_px_wide = board.width * PIXELS_PER_TILE * WINDOW_SCALE;
    let num_px_high = board.height * PIXELS_PER_TILE * WINDOW_SCALE;
    let window_size = (num_px_wide, num_px_high);

    let mut window: PistonWindow = WindowSettings::new("p a c m a n", window_size)
        .exit_on_esc(true)
        .graphics_api(Api::opengl(3, 2))
        .resizable(false)
        .decorated(false)
        .build()
        .unwrap();

    let player_start = Vec2::new(
        (board.width / 2 * PIXELS_PER_TILE) as f32,
        ((board.height - 8) * PIXELS_PER_TILE) as f32,
    );
    let player = CharacterState {
        position: player_start,
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
        if let Some(_) = e.render_args() {
            //          texture.update(&mut window.encoder, &pixel_buffer).unwrap();
            window.draw_2d(&e, |context, g, _| {
                let clear_color = [0.2, 0.2, 0.2, 1.0];
                clear(clear_color, g);

                // draw board
                let board = &gamestate.board;
                for row in 0..board.height {
                    for col in 0..board.width {
                        if let Some(tile) = board.tile_from_row_and_col(row, col) {
                            // get the color for the tile
                            let black = [0.0, 0.0, 0.0, 1.0];
                            let tunnel_color = [0.2, 0.2, 0.2, 1.0];
                            let wall_color = [0.0, 0.0, 0.5, 1.0];
                            let color = if !tile.is_traversable {
                                wall_color
                            } else if tile.is_tunnel {
                                tunnel_color
                            } else {
                                black
                            };

                            // this is constant for all tiles
                            const TILE_EXTENTS: Vec2 = Vec2 {
                                x: PIXELS_PER_TILE as f32 * 0.5,
                                y: PIXELS_PER_TILE as f32 * 0.5,
                            };
                            let tile_pos = Vec2 {
                                x: col as f32 * PIXELS_PER_TILE as f32,
                                y: row as f32 * PIXELS_PER_TILE as f32,
                            };
                            let rect = [ tile_pos.x as f64, tile_pos.y as f64, TILE_EXTENTS.x as f64 * 2.0, TILE_EXTENTS.y as f64 * 2.0 ];
                            let transform = context
                                .transform
                                .scale(WINDOW_SCALE as f64, WINDOW_SCALE as f64);

                            // draw tile
                            rectangle(color, rect, transform, g);

                            // draw pellet if there is one
                            if tile.has_pellet {
                                // draw a PELLET
                                const PELLET_COLOR: [f32; 4] = [0.8,0.8,0.8,1.0];
                                let pellet_extents = TILE_EXTENTS * 0.25;
                                let pellet_pos = tile_pos + TILE_EXTENTS - pellet_extents;
                                let pellet = Ellipse::new(PELLET_COLOR);
                                let rect: [f64;4] = [
                                    pellet_pos.x as f64,
                                    pellet_pos.y as f64,
                                    pellet_extents.x as f64 * 2.0,
                                    pellet_extents.y as f64 * 2.0,
                                ];

                                pellet.draw(rect, &Default::default(), transform, g);
                            }
                            if tile.has_power_pellet {
                                // draw a POWER_PELLET
                                // - the only thing that changes here is `scale`
                                //   everything else is the same as draing pellet.
                                //   good candidate to pull out
                                const PELLET_COLOR: [f32; 4] = [0.8,0.8,0.8,1.0];
                                let pellet_extents = TILE_EXTENTS * 0.5;
                                let pellet_pos = tile_pos + TILE_EXTENTS - pellet_extents;
                                let pellet = Ellipse::new(PELLET_COLOR);
                                let rect: [f64;4] = [
                                    pellet_pos.x as f64,
                                    pellet_pos.y as f64,
                                    pellet_extents.x as f64 * 2.0,
                                    pellet_extents.y as f64 * 2.0,
                                ];

                                pellet.draw(rect, &Default::default(), transform, g);
                            }
                        } else {
                            println!("failed to get tile for (row,col): ({},{})", row, col);
                            assert!(false);
                        }
                    }
                }

                // draw grid
                let grid = grid::Grid {
                    cols: gamestate.board.width,
                    rows: gamestate.board.height,
                    units: (PIXELS_PER_TILE * WINDOW_SCALE) as f64,
                };
                let line = Line {
                    color: [0.8, 0.8, 0.8, 1.0], // <--- grey
                    radius: 0.5,
                    shape: line::Shape::Round,
                };
                grid.draw(&line, &context.draw_state, context.transform, g);
                // draw dots
                // draw fruits
                // draw score
                {
                    // draw player
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
