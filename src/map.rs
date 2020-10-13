use rltk::{RandomNumberGenerator};
use std::cmp::{min, max};
use super::{Rect};

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
            tiles: vec![Tile::Wall; (width * height) as usize]
        };

        let mut rooms: Vec<Rect> = Vec::new();
        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 4;
        const MAX_SIZE: i32 = 10;

        let mut rng = RandomNumberGenerator::new();

        for _ in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, width - w - 1) - 1;
            let y = rng.roll_dice(1, height - h - 1) - 1;
            let new_room = Rect::new(x, y, w, h);
            let mut ok = true;
            
            for other_room in rooms.iter() {
                if new_room.intersect(other_room) { ok = false }
            }

            if ok {
                apply_room_to_map(&new_room, &mut map);

                rooms.push(new_room);
            }
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

fn apply_room_to_map(room: &Rect, map: &mut Map) {
    for y in room.y1 +1 ..= room.y2 {
        for x in room.x1 + 1 ..= room.x2 {
            map.tiles[xy_idx(map.width, x, y)] = Tile::Floor;
        }
    }
}

fn apply_horizontal_tunnel(map: &mut Map, x1: i32, x2: i32, y: i32) {
    for x in min(x1, x2) ..= max(x1, x2) {
        let idx = xy_idx(map.width, x, y);
        if idx > 0 && idx < (map.width * map.height) as usize {
            map.tiles[idx] = Tile::Floor;
        }
    }
}

fn apply_vertical_tunnel(map: &mut Map, y1: i32, y2: i32, x: i32) {
    for y in min(y1, y2) ..= max(y1, y2) {
        let idx = xy_idx(map.width, x, y);
        if idx > 0 && idx < (map.width * map.height) as usize {
            map.tiles[idx] = Tile::Floor;
        }
    }
}