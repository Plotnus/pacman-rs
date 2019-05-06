//
// P = power pellet
// p = pellet
// S = pac_start
// G = ghost_pen
// C = clyde start
// B = blinky start
const MAZE_DEF: &str = "\
_XXXXXXXXXXXXXXXXXXXXXXXXXXXX\
_X............XX............X\
_X.XXXX.XXXXX.XX.XXXXX.XXXX.X\
_XoXXXX.XXXXX.XX.XXXXX.XXXXoX\
_X.XXXX.XXXXX.XX.XXXXX.XXXX.X\
_X..........................X\
_X.XXXX.XX.XXXXXXXX.XX.XXXX.X\
_X.XXXX.XX.XXXXXXXX.XX.XXXX.X\
_X......XX....XX....XX......X\
_XXXXXX.XXXXX XX XXXXX.XXXXXX\
_XXXXXX.XXXXX XX XXXXX.XXXXXX\
_XXXXXX.XX          XX.XXXXXX\
_XXXXXX.XX XXX--XXX XX.XXXXXX\
_XXXXXX.XX X      X XX.XXXXXX\
_      .   X      X   .      \
_XXXXXX.XX X      X XXXXXXXXX\
_XXXXXX.XX XXXXXXXX XXXXXXXXX\
_XXXXXX.XX          XXXXXXXXX\
_XXXXXX.XX XXXXXXXX XXXXXXXXX\
_XXXXXX.XX XXXXXXXX XXXXXXXXX\
_X............  ............X\
_X.XXXX.XXXXX.XX.XXXXXXXXXX.X\
_X.XXXX.XXXXX.XX.XXXXXXXXXX.X\
_Xo..XX.......  .......XX..oX\
_XXX.XX.XX.XXXXXXXX.XX.XX.XXX\
_XXX.XX.XX.XXXXXXXX.XX.XX.XXX\
_X......XX....XX....XX......X\
_X.XXXXXXXXXX.XX.XXXXXXXXXX.X\
_X.XXXXXXXXXX.XX.XXXXXXXXXX.X\
_X..........................X\
_XXXXXXXXXXXXXXXXXXXXXXXXXXXX\
";
////////////////////////////////////////////////////////////////////////////////
pub struct Tile {
    pub has_pellet: bool,
    pub has_power_pellet: bool,
    pub is_traversable: bool,
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
            if c == '_' { continue ;}
            let is_traversable = c == 'X';
            let has_pellet = c == '.';
            let has_power_pellet = c == 'o';
            let tile = Tile {
                has_pellet, has_power_pellet, is_traversable
            };
            tiles.push(tile);
        }

        Board { tiles, width: width as u32, height: height as u32}
    }

    pub fn tile_from_row_and_col(&self, row: u32, col: u32) -> Option<&Tile> {
        assert!(row < self.height && col < self.width);
        let index = (row * self.width + col) as usize;
        self.tiles.get(index)
    }
}
