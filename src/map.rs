#[derive(PartialEq, Copy, Clone)]
pub enum Tile {
    Wall, Floor
}

pub struct Map {
    width: i32,
    height: i32,
    tiles: Vec<Tile>,
}

impl Map {
    pub fn get_size(&self) -> (i32, i32) {
        (self.width, self.height)
    }

    pub fn new(width: i32, height: i32) -> Map {
        let mut map = Map {
            width,
            height,
            tiles: vec![Tile::Floor; (width * height) as usize]
        };

        for x in 0..width {
            map.tiles[xy_idx(width, x, 0)] = Tile::Wall;
            map.tiles[xy_idx(width, x, height - 1)] = Tile::Wall;
        }

        for y in 0..height {
            map.tiles[xy_idx(width, 0, y)] = Tile::Wall;
            map.tiles[xy_idx(width, width, 0)] = Tile::Wall;
        }

        map
    }

    pub fn get_tile(&self, x: i32, y: i32) -> Tile {
        self.tiles[xy_idx(self.width, x, y)]
    }
}

fn xy_idx(width: i32, x: i32, y: i32) -> usize {
    (y * width + x) as usize
}

fn new_map_rooms_and_corridoors(width:i32, height:i32) -> Vec<Tile> {
    let mut map = vec![Tile::Wall; (width*height) as usize];

    map
}