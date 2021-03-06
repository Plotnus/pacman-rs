use crate::vec2::Vec2;
// X = wall
// . = pellet
// o = power pellet
// t = tunnel
const MAZE_DEF: &str = "\
                    XXXXXXXXXXXXXXXXXXXXXXXXXXXX\
                    X............XX............X\
                    X.XXXX.XXXXX.XX.XXXXX.XXXX.X\
                    XoXXXX.XXXXX.XX.XXXXX.XXXXoX\
                    X.XXXX.XXXXX.XX.XXXXX.XXXX.X\
                    X..........................X\
                    X.XXXX.XX.XXXXXXXX.XX.XXXX.X\
                    X.XXXX.XX.XXXXXXXX.XX.XXXX.X\
                    X......XX....XX....XX......X\
                    XXXXXX.XXXXX XX XXXXX.XXXXXX\
                    XXXXXX.XXXXX XX XXXXX.XXXXXX\
                    XXXXXX.XX          XX.XXXXXX\
                    XXXXXX.XX XXX--XXX XX.XXXXXX\
                    XXXXXX.XX X      X XX.XXXXXX\
                    tttttt.   X      X   .tttttt\
                    XXXXXX.XX X      X XX.XXXXXX\
                    XXXXXX.XX XXXXXXXX XX.XXXXXX\
                    XXXXXX.XX          XX.XXXXXX\
                    XXXXXX.XX XXXXXXXX XX.XXXXXX\
                    XXXXXX.XX XXXXXXXX XX.XXXXXX\
                    X............XX............X\
                    X.XXXX.XXXXX.XX.XXXXX.XXXX.X\
                    X.XXXX.XXXXX.XX.XXXXX.XXXX.X\
                    Xo..XX.......  .......XX..oX\
                    XXX.XX.XX.XXXXXXXX.XX.XX.XXX\
                    XXX.XX.XX.XXXXXXXX.XX.XX.XXX\
                    X......XX....XX....XX......X\
                    X.XXXXXXXXXX.XX.XXXXXXXXXX.X\
                    X.XXXXXXXXXX.XX.XXXXXXXXXX.X\
                    X..........................X\
                    XXXXXXXXXXXXXXXXXXXXXXXXXXXX\
                    ";

pub struct Board {
    pub is_traversable: Vec<bool>,
    pub is_tunnel: Vec<bool>,
    pub has_pellet: Vec<bool>,
    pub has_power_pellet: Vec<bool>,

    pub width: usize,
    pub height: usize,
    pub num_tiles: usize,
}


pub struct BoardPos {
    pub x: usize,
    pub y: usize,
}


impl From<Vec2> for BoardPos {
    fn from(p: Vec2) -> BoardPos {
        BoardPos {
            x: (p.x / Board::TILE_WIDTH) as usize,
            y: (p.y / Board::TILE_WIDTH) as usize,
        }
    }
}


impl Board {
    pub const TILE_WIDTH: f32 = 8.0;
    pub fn new() -> Board {
        let width = 28;
        let height = 31;
        let num_tiles = width * height;

        let mut is_traversable = Vec::with_capacity(num_tiles);
        let mut has_power_pellet = Vec::with_capacity(num_tiles);
        let mut has_pellet = Vec::with_capacity(num_tiles);
        let mut is_tunnel = Vec::with_capacity(num_tiles);

        for c in MAZE_DEF.chars() {
            is_traversable.push(c != 'X');
            has_power_pellet.push(c == 'o');
            has_pellet.push(c == '.');
            is_tunnel.push(c == 't');
        }

        assert_eq!(num_tiles, is_traversable.len());
        assert_eq!(num_tiles, is_tunnel.len());
        assert_eq!(num_tiles, has_pellet.len());
        assert_eq!(num_tiles, has_power_pellet.len());

        Board {
            is_traversable,
            is_tunnel,
            has_pellet,
            has_power_pellet,
            width,
            height,
            num_tiles,
        }
    }

    pub fn get_local_pos_of_tile(&self, h: usize) ->  Vec2 {
        Vec2 {
            x: (h % self.width) as f32 * Board::TILE_WIDTH,
            y: (h / self.width) as f32 * Board::TILE_WIDTH,
        }

    }
    pub fn get_board_pos_of_tile(&self, h: usize) ->  BoardPos {
        assert!(h < self.num_tiles);
        BoardPos {
            x: h % self.width,
            y: h / self.width,
        }
    }
    pub fn get_tile_of_board_pos(&self, board_pos: BoardPos) ->  usize {
        assert!(board_pos.x < self.width);
        assert!(board_pos.y < self.height);

        board_pos.y * self.width + board_pos.x
    }

    pub fn tile_is_traversable(&self, h: usize) -> bool {
        assert!(h < self.is_traversable.len());
        self.is_traversable[h]
    }

    pub fn tile_is_tunnel(&self, h: usize) -> bool {
        assert!(h < self.is_tunnel.len());
        self.is_tunnel[h]
    }

    pub fn tile_has_pellet(&self, h: usize) -> bool {
        assert!(h < self.has_pellet.len());
        self.has_pellet[h]
    }

    pub fn tile_has_power_pellet(&self, h: usize) -> bool {
        assert!(h < self.has_power_pellet.len());
        self.has_power_pellet[h]
    }
}
