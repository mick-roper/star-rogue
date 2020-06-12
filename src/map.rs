use rltk::{Rltk, RandomNumberGenerator, BaseMap, Algorithm2D, Point, RGB, Console};
use super::{Rect};
use std::cmp::{max, min};

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall, Floor
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32
}

impl Map {
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    fn add_room(&mut self, room: &Rect) {
        for y in room.y1 +1 ..= room.y2 {
            for x in room.x1 + 1 ..= room.x2 {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn add_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        let boundary = self.width as usize * self.height as usize;
        for x in min(x1, x2) ..= max(x1, x2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < boundary {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }
    
    fn add_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        let boundary = self.width as usize * self.height as usize;
        for y in min(y1, y2) ..= max(y1, y2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < boundary {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx] == TileType::Wall
    }
}

pub fn new_map(width: i32, height: i32) -> Map {
    let mut map = Map {
        tiles: vec![TileType::Wall; 80*50],
        rooms: Vec::new(),
        width: width,
        height: height
    };

    const MAX_ROOMS: i32 = 30;
    const MIN_ROOM_SIZE: i32 = 6;
    const MAX_ROOM_SIZE: i32 = 10;

    let mut rng = RandomNumberGenerator::new();

    for _ in 0..MAX_ROOMS {
        let w = rng.range(MIN_ROOM_SIZE, MAX_ROOM_SIZE);
        let h = rng.range(MIN_ROOM_SIZE, MAX_ROOM_SIZE);
        let x = rng.roll_dice(1, 80 - w - 1) - 1;
        let y = rng.roll_dice(1, 50 - h - 1) - 1;
        let new_room = Rect::new(x, y, w, h);
        
        let mut ok = true;
        for other_room in map.rooms.iter() {
            if new_room.intersect(other_room) { ok = false }
        }

        if ok {
            map.add_room(&new_room);

            if !map.rooms.is_empty() {
                let (new_x, new_y) = new_room.centre();
                let (prev_x, prev_y) = map.rooms[map.rooms.len() - 1].centre();
                if rng.range(1, 2) == 1 {
                    map.add_horizontal_tunnel(prev_x, new_x, prev_y);
                    map.add_vertical_tunnel(prev_y, new_y, new_x);
                } else {
                    map.add_vertical_tunnel(prev_y, new_y, new_x);
                    map.add_horizontal_tunnel(prev_x, new_x, prev_y);
                }
            }

            map.rooms.push(new_room);
        }
    }

    map
}

pub fn draw_map(map: &[TileType], ctx: &mut Rltk) {
    let mut y = 0;
    let mut x = 0;

    let floor_colour = RGB::from_f32(0.5, 0.5, 0.5);
    let wall_colour = RGB::from_f32(0.0, 1.0, 0.0);
    let black = RGB::from_f32(0., 0., 0.);
    let dot = rltk::to_cp437('.');
    let hash = rltk::to_cp437('#');

    for tile in map.iter() {
        match tile {
            TileType::Floor => {
                ctx.set(x, y, floor_colour, black, dot);
            },
            TileType::Wall => {
                ctx.set(x, y, wall_colour, black, hash);
            }
        }

        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}