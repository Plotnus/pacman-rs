use piston_window::*;

mod board;
mod input;
mod vec2;
use crate::board::*;
use crate::input::*;

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
    window.set_ups(60);
    window.set_max_fps(60);

    let x = ((board.width / 2) as f32) * Board::TILE_WIDTH;
    let y = ((board.height / 2) as f32 + 5.5) * Board::TILE_WIDTH;
    let player_start = Vec2::new(x, y);
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

    let frames_per_second = 60.0;
    let frame_duration_target = std::time::Duration::from_secs_f64(1.0 / frames_per_second);
    dbg!(frame_duration_target);


    while !window.should_close() {

        let frame_start_time = std::time::Instant::now();

        let input_stream = get_input_stream(&mut window);

        gamestate = process_input_stream(gamestate, input_stream);

        gamestate = step(gamestate);

        render(&mut window, &gamestate);

        // IDLE - IDLE - IDLE
        let frame_duration = frame_start_time.elapsed();
        println!("FrameDuration: {}ns, TargetDuration: {}ns, OverBudget: {}"
            ,frame_duration.as_nanos()
            ,frame_duration_target.as_nanos()
            ,frame_duration > frame_duration_target
        );
    }
}

////////////////////////////////////////////////////////////////////////////////
struct World {}
impl World {
    const LEFT: Vec2 = Vec2::new(-1_f32, 0_f32);
    const RIGHT: Vec2 = Vec2::new(1_f32, 0_f32);
    const UP: Vec2 = Vec2::new(0_f32, -1_f32);
    const DOWN: Vec2 = Vec2::new(0_f32, 1_f32);
}
////////////////////////////////////////////////////////////////////////////////
fn process_input_stream(mut gamestate: GameState, input_queue: Vec<GameInput>) -> GameState{
    for input in input_queue {
        gamestate.player.move_dir = match input {
            GameInput::Down => World::DOWN,
            GameInput::Up => World::UP,
            GameInput::Right => World::RIGHT,
            GameInput::Left => World::LEFT,
            _ => gamestate.player.move_dir,
        };

        if input == GameInput::Step {
            gamestate.ready_to_process_turn = true;
        }
    }
    gamestate
}
fn get_input_stream(window: &mut PistonWindow) -> Vec<GameInput> {
    let mut piston_keyboard_presses = Vec::new();
    while let Some(e) = window.poll_event() {
        match e {
            Event::Input(piston_input, _time) => {
                if let Input::Button(button_args) = piston_input {
                    if button_args.state == ButtonState::Press {
                        if let Button::Keyboard(key) = button_args.button {
                            piston_keyboard_presses.push(key);
                        }
                    }
                }
            },
            _ => {
                println!("ERROR: We should only get input events in this loop, got event {:?}", &e);
                assert!(false);
            },
        }
    }

    piston_keyboard_presses.iter()
        .map(|piston_input| { GameInput::from(piston_input) })
        .collect()
}
////////////////////////////////////////////////////////////////////////////////
fn step(mut gamestate: GameState) -> GameState {
    if !gamestate.ready_to_process_turn {
        return gamestate;
    }
    gamestate.ready_to_process_turn = false;


    // get the position we would move to
    let board_pos_to_check = {
        let mut pos = gamestate.player.position;
        pos = pos + gamestate.player.move_dir * Board::TILE_WIDTH;
        BoardPos::from(pos)
    };

    // board_pos -> tile_handle
    let tile_handle = gamestate.board.get_tile_of_board_pos(board_pos_to_check);


    if gamestate.board.tile_is_traversable(tile_handle) {
        gamestate.player.position += gamestate.player.move_dir;
    }

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

    gamestate
}

fn render(window: &mut PistonWindow, gamestate: &GameState) {
    if let Some(e) = window.next() {
        window.draw_2d(&e, |context, g, _| {
            render_gamestate(&gamestate, context, g);
        });
    }
}

fn render_gamestate(gamestate: &GameState, context :Context, g: &mut G2d) {
    // texture.update(&mut window.encoder, &pixel_buffer).unwrap();
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
            let pos = board.get_local_pos_of_tile(h);
            const WALL_COLOR: [f32; 4] = [0.0, 0.0, 0.6, 1.0];
            draw_tile(&pos, WALL_COLOR, transform, g);
        }
    }

    // draw tunnels
    for h in 0..board.num_tiles {
        if board.tile_is_tunnel(h) {
            const TUNNEL_COLOR: [f32; 4] = [0.2, 0.2, 0.2, 1.0];
            let pos = board.get_local_pos_of_tile(h);
            draw_tile(&pos, TUNNEL_COLOR, transform, g);
        }
    }

    // draw pellets
    for h in 0..board.num_tiles {
        if board.tile_has_pellet(h) {
            const PELLET_COLOR: [f32; 4] = [0.8, 0.8, 0.8, 1.0];
            const SCALE: f32 = 0.25;
            let pos = board.get_board_pos_of_tile(h);
            draw_circle(pos, PELLET_COLOR, SCALE, transform, g);
        }
    }

    // draw power_pellets
    for h in 0..board.num_tiles {
        if board.tile_has_power_pellet(h) {
            const PELLET_COLOR: [f32; 4] = [0.8, 0.8, 0.8, 1.0];
            const SCALE: f32 = 0.5;
            let pos = board.get_board_pos_of_tile(h);
            draw_circle(pos, PELLET_COLOR, SCALE, transform, g);
        }
    }

    {
        // draw grid
        let grid = grid::Grid {
            cols: gamestate.board.width as u32,
            rows: gamestate.board.height as u32,
            units: (Board::TILE_WIDTH * WINDOW_SCALE as f32) as f64,
        };
        let line = Line {
            color: [0.8, 0.8, 0.8, 1.0], // <--- grey
            radius: 0.5,
            shape: line::Shape::Round,
        };
        grid.draw(&line, &context.draw_state, context.transform, g);
    }

    {
        // draw player
        // casting now saves some casts later
        let x = gamestate.player.position.x as f64;
        let y = gamestate.player.position.y as f64;

        // draw tile pos of pacman
        let board_pos = BoardPos::from(gamestate.player.position);
        let tile = gamestate.board.get_tile_of_board_pos(board_pos);
        let local_pos = gamestate.board.get_local_pos_of_tile(tile);
        let yellow = [1.0, 1.0, 0.0, 0.5];
        draw_tile(&local_pos, yellow, transform, g);
        // draw pixel pos of pacman
        let red = [1.0, 0.0, 0.0, 1.0];
        let rect = [x, y, 1.0, 1.0];
        rectangle(red, rect, transform, g);

        let center_of_tile = {
            let board_pos = BoardPos::from(gamestate.player.position);
            let mut v = Vec2::new(board_pos.x as f32, board_pos.y as f32);
            v.x += 0.5;
            v.y += 0.5;
            v *= Board::TILE_WIDTH;
            v
        };

        // offset from center of tile
        let color = [0.0, 0.0, 0.0, 1.0];
        let scale = 1.2;
        let offset = 1.8;
        let forward = gamestate.player.move_dir;

        let transform = transform
            .trans(center_of_tile.x as f64, center_of_tile.y as f64)
            .orient(forward.x as f64, forward.y as f64)
            .zoom(scale)
            .trans(offset, 0.0);

        // we're using a triangle so the next few bits of code are about
        // a) defining the points of our triangle
        // b) drawing the lines to make the triangle
        let a = [1.0, 0.0];
        let b = [0.0, -0.8];
        let c = [0.0, 0.8];

        // draw
        let line = Line {
            color: color,
            radius: 0.4,
            shape: line::Shape::Round,
        };
        line.draw_from_to(a, b, &context.draw_state, transform, g);
        line.draw_from_to(b, c, &context.draw_state, transform, g);
        line.draw_from_to(c, a, &context.draw_state, transform, g);
    }

}
////////////////////////////////////////////////////////////////////////////////
fn draw_tile<G>(pos: &Vec2, color: [f32; 4], transform: math::Matrix2d, g: &mut G)
where
    G: Graphics,
{
    let rect = [0.0, 0.0, Board::TILE_WIDTH as f64, Board::TILE_WIDTH as f64];
    let transform = transform.trans(pos.x as f64, pos.y as f64);
    rectangle(color, rect, transform, g);
}

fn draw_circle<G>(pos: BoardPos, color: [f32; 4], scale: f32, transform: math::Matrix2d, g: &mut G)
where
    G: Graphics,
{
    let w = PIXELS_PER_TILE as f64 * scale as f64;
    let h = w;

    let x = (pos.x * PIXELS_PER_TILE) as f64 + PIXELS_PER_TILE as f64 * 0.5 - w * 0.5;
    let y = (pos.y * PIXELS_PER_TILE) as f64 + PIXELS_PER_TILE as f64 * 0.5 - h * 0.5;
    let rect: [f64; 4] = [x as f64, y as f64, w as f64, h as f64];
    Ellipse::new(color).draw(rect, &Default::default(), transform, g);
}
