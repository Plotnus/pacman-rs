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
                    XXXXXX.XX X      X XXXXXXXXX\
                    XXXXXX.XX XXXXXXXX XXXXXXXXX\
                    XXXXXX.XX          XXXXXXXXX\
                    XXXXXX.XX XXXXXXXX XXXXXXXXX\
                    XXXXXX.XX XXXXXXXX XXXXXXXXX\
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
    pub width: u32,
    pub height: u32,
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
            width: width as u32,
            height: height as u32,
        }
    }

    pub fn tile_from_row_and_col(&self, row: u32, col: u32) -> Option<&Tile> {
        assert!(row < self.height && col < self.width);
        let index = (row * self.width + col) as usize;
        self.tiles.get(index)
    }
}
