use rltk::{RandomNumberGenerator, Algorithm2D, BaseMap};
use std::cmp::{min, max};
use super::{Rect};

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall, Floor
}

pub struct Map {
    width: i32,
    height: i32,
    tiles: Vec<TileType>,
    rooms: Vec<Rect>,
}

impl Map {
    pub fn get_size(&self) -> (i32, i32) {
        (self.width, self.height)
    }

    pub fn get_room(&self, index: i32) -> Rect {
        self.rooms[index as usize]
    }

    pub fn new(width: i32, height: i32) -> Map {
        let mut map = Map {
            width,
            height,
            tiles: vec![TileType::Wall; (width * height) as usize],
            rooms: Vec::new(),
        };

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
            
            for other_room in map.rooms.iter() {
                if new_room.intersect(other_room) { ok = false }
            }

            if ok {
                map.apply_room_to_map(&new_room);

                if !map.rooms.is_empty() {
                    let (new_x, new_y) = new_room.centre();
                    let (prev_x, prev_y) = map.rooms[map.rooms.len() - 1].centre();
                    if rng.range(0, 2) == 1 {
                        map.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                        map.apply_vertical_tunnel(prev_y, new_y, new_x);
                    } else {
                        map.apply_vertical_tunnel(prev_y, new_y, prev_x);
                        map.apply_horizontal_tunnel(prev_x, new_x, new_y);
                    }
                }

                map.rooms.push(new_room);
            }
        }

        map
    }

    pub fn get_tile(&self, x: i32, y: i32) -> TileType {
        self.tiles[self.xy_idx(x, y)]
    }

    fn apply_room_to_map(&mut self, room: &Rect) {
        for y in room.y1 +1 ..= room.y2 {
            for x in room.x1 + 1 ..= room.x2 {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = TileType::Floor;
            }
        }
    }
    
    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2) ..= max(x1, x2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < (self.width * self.height) as usize {
                self.tiles[idx] = TileType::Floor;
            }
        }
    }
    
    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2) ..= max(y1, y2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < (self.width * self.height) as usize {
             self.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y * self.width + x) as usize
    }
}