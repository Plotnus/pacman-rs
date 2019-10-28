//
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
                    X............  ............X\
                    X.XXXX.XXXXX.XX.XXXXXXXXXX.X\
                    X.XXXX.XXXXX.XX.XXXXXXXXXX.X\
                    Xo..XX.......  .......XX..oX\
                    XXX.XX.XX.XXXXXXXX.XX.XX.XXX\
                    XXX.XX.XX.XXXXXXXX.XX.XX.XXX\
                    X......XX....XX....XX......X\
                    X.XXXXXXXXXX.XX.XXXXXXXXXX.X\
                    X.XXXXXXXXXX.XX.XXXXXXXXXX.X\
                    X..........................X\
                    XXXXXXXXXXXXXXXXXXXXXXXXXXXX\
                    ";
////////////////////////////////////////////////////////////////////////////////
pub struct Tile {
    pub has_pellet: bool,
    pub has_power_pellet: bool,
    pub is_traversable: bool,
    pub is_tunnel: bool,
}

////////////////////////////////////////////////////////////////////////////////
pub struct Board {
    pub tiles: Vec<Tile>,
    pub width: usize,
    pub height: usize,
}
// tiles have a board position
// this is different than pixel position
pub struct BoardPos {
    pub x: usize,
    pub y: usize,
}

impl Board {
    pub fn new() -> Board {
        let mut tiles: Vec<Tile> = Vec::new();
        let width = 28;
        let height = 31;
        tiles.reserve(width * height);
        for c in MAZE_DEF.chars() {
            let tile = Tile {
                has_pellet: c == '.',
                has_power_pellet: c == 'o',
                is_traversable: c != 'X',
                is_tunnel: c == 't',
            };
            tiles.push(tile);
        }

        Board {
            tiles,
            width: width,
            height: height,
        }
    }

    pub fn get_tile(&self, h: usize) -> &Tile {
        assert!(h < self.tiles.len());
        &self.tiles[h]
    }

    pub fn get_board_pos_of_tile(&self, h: usize) ->  BoardPos {
        assert!(h < self.tiles.len());
        BoardPos {
            x: h % self.width,
            y: h / self.width,
        }
    }

    pub fn tile_is_traversable(&self, h: usize) -> bool {
        self.get_tile(h).is_traversable
    }

    pub fn tile_is_tunnel(&self, h: usize) -> bool {
        self.get_tile(h).is_tunnel
    }

    pub fn tile_has_pellet(&self, h: usize) -> bool {
        self.get_tile(h).has_pellet
    }

    pub fn tile_has_power_pellet(&self, h: usize) -> bool {
        self.get_tile(h).has_power_pellet
    }
}
