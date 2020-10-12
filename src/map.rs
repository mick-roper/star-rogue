pub enum Tile {
    Wall, Floor
}

pub struct Map {
    width: i32,
    height: i32,
    tiles: Vec<Tile>,
}

impl Map {
    fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y * width as usize) + x as usize
    }

    pub fn new(width: i32, height: i32) -> Map {
        let mut map = Map {
            width,
            height,
            tiles: vec![Tile::Floor, width * height],
        };

        for x in 0..width {
            map[map.xy_idx(x, 0)] = Tile::Wall;
            map[map.xy_idx(x, height - 1)] = Tile::Wall;
        }

        for y in 0..width {
            map[map.xy_idx(0, y)] = Tile::Wall;
            map[map.xy_idx(width - 1, 0)] = Tile::Wall;
        }

        map
    }
}