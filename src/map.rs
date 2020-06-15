use rltk::{RandomNumberGenerator, BaseMap, Algorithm2D, Point, Rltk, RGB, Console};
use super::{Rect, World};
use std::cmp::{max, min};

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall, Floor
}

#[derive(Default)]
pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>
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
    
    fn is_exit_valid(&self, x: i32, y: i32) -> bool {
        if x < 1 || x > self.width || y < 1 || y > self.height - 1 { return false; }
        let idx = self.xy_idx(x, y);
        self.tiles[idx as usize] != TileType::Wall
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

    fn get_available_exits(&self, idx: usize) -> Vec<(usize, f32)> {
        let mut exits : Vec<(usize, f32)> = Vec::new();
        let x = idx as i32 % self.width;
        let y = idx as i32 / self.width;
        let w = self.width as usize;

        // cardinal directions
        if self.is_exit_valid(x-1, y) { exits.push((idx-1, 1.0)) };
        if self.is_exit_valid(x+1, y) { exits.push((idx+1, 1.0)) };
        if self.is_exit_valid(x, y-1) { exits.push((idx-w, 1.0)) };
        if self.is_exit_valid(x, y+1) { exits.push((idx+w, 1.0)) };

        // diagonals
        if self.is_exit_valid(x-1, y-1) { exits.push(((idx-w)-1, 1.45)) };
        if self.is_exit_valid(x+1, y-1) { exits.push(((idx-w)+1, 1.45)) };
        if self.is_exit_valid(x-1, y+1) { exits.push(((idx+w)-1, 1.45)) };
        if self.is_exit_valid(x+1, y+1) { exits.push(((idx+w)+1, 1.45)) };

        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        let w = self.width as usize;
        let p1 = Point::new(idx1 % w, idx1 / w);
        let p2 = Point::new(idx2 % w, idx2 / w);
        rltk::DistanceAlg::Pythagoras.distance2d(p1, p2)
    }
}

pub fn new_map(width: i32, height: i32) -> Map {
    let dimensions = (width * height) as usize;
    let mut map = Map {
        tiles: vec![TileType::Wall; dimensions],
        rooms: Vec::new(),
        width: width,
        height: height,
        revealed_tiles: vec![false; dimensions],
        visible_tiles: vec![false; dimensions]
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

const FLOOR_COLOUR: RGB = RGB{b: 127.5, g: 127.5, r: 0.0};
const WALL_COLOUR: RGB = RGB{b: 0.0, g: 255.0, r: 0.0};
const BLACK: RGB = RGB{b: 0.0, g: 0.0, r: 0.0};

pub fn draw_map(ecs: &World, ctx: &mut Rltk) {
    let map = ecs.fetch::<Map>();

    let mut y = 0;
    let mut x = 0;

    for (idx, tile) in map.tiles.iter().enumerate() {
        if map.revealed_tiles[idx] {
            let glyph;
            let mut fg;

            match tile {
                TileType::Floor => {
                    glyph = rltk::to_cp437('.');
                    fg = FLOOR_COLOUR
                },
                TileType::Wall => {
                    glyph = rltk::to_cp437('#');
                    fg = WALL_COLOUR
                }
            }

            if !map.visible_tiles[idx] { fg = fg.to_greyscale() }

            ctx.set(x, y, fg, BLACK, glyph);
        }

        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}