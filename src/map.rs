use super::Rect;
use rltk::{Algorithm2D, BaseMap, Point, RandomNumberGenerator};
use std::cmp::{max, min};
use specs::{Entity};

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

#[derive(Default)]
pub struct Map {
    width: i32,
    height: i32,
    tiles: Vec<TileType>,
    rooms: Vec<Rect>,
    revealed_tiles: Vec<bool>,
    visible_tiles: Vec<bool>,
    blocked_tiles: Vec<bool>,
    tile_content: Vec<Vec<Entity>>,
}

impl Map {
    pub fn new(width: i32, height: i32) -> Map {
        let mut map = Map {
            width,
            height,
            tiles: vec![TileType::Wall; (width * height) as usize],
            rooms: Vec::new(),
            revealed_tiles: vec![false; (width * height) as usize],
            visible_tiles: vec![false; (width * height) as usize],
            blocked_tiles: vec![false; (width * height) as usize],
            tile_content: vec![Vec::new(); (width * height) as usize],
        };

        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 4;
        const MAX_SIZE: i32 = 10;

        let mut rng = RandomNumberGenerator::new();

        // add rooms
        for _ in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, width - w - 1) - 1;
            let y = rng.roll_dice(1, height - h - 1) - 1;
            let new_room = Rect::new(x, y, w, h);
            let mut ok = true;
            for other_room in map.rooms.iter() {
                if new_room.intersect(other_room) {
                    ok = false
                }
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

        map.update_blocked_tiles();

        map
    }

    pub fn add_tile_content(&mut self, x: i32, y: i32, entity: Entity) {
        let idx = self.xy_idx(x, y);
        self.tile_content[idx].push(entity);
    }

    // pub fn get_tile_content(&self, x: i32, y: i32) -> &Vec<Entity> {
    //     let idx = self.xy_idx(x, y);
    //     &self.tile_content[idx];
    // }

    pub fn clear_content_index(&mut self) {
        for content in self.tile_content.iter_mut() {
            content.clear();
        }
    }

    pub fn tile_is_blocked(&self, x: i32, y: i32) -> bool {
        let idx = self.xy_idx(x, y);
        self.blocked_tiles[idx]
    }

    pub fn update_blocked_tiles(&mut self) {
        for (i, tile) in self.tiles.iter_mut().enumerate() {
            self.blocked_tiles[i] = *tile == TileType::Wall;
        }
    }

    pub fn set_tile_as_blocked(&mut self, x: i32, y: i32) {
        let idx = self.xy_idx(x, y);
        self.blocked_tiles[idx] = true;
    }

    pub fn get_room(&self, index: i32) -> Rect {
        self.rooms[index as usize]
    }

    pub fn get_room_count(&self) -> i32 {
        self.rooms.len() as i32
    }

    pub fn get_dimensions(&self) -> (i32, i32) {
        (self.width, self.height)
    }

    pub fn tile_is_visible(&self, x: i32, y: i32) -> bool {
        self.visible_tiles[self.xy_idx(x, y)]
    }

    pub fn tile_is_revealed(&self, x: i32, y: i32) -> bool {
        self.revealed_tiles[self.xy_idx(x, y)]
    }

    pub fn reveal_tile(&mut self, x: i32, y: i32) {
        let idx = self.xy_idx(x, y);
        self.revealed_tiles[idx] = true;
    }

    pub fn clear_visible_tiles(&mut self) {
        for t in self.visible_tiles.iter_mut() {
            *t = false
        }
    }

    pub fn mark_tile_as_visible(&mut self, x: i32, y: i32) {
        let idx = self.xy_idx(x, y);
        self.visible_tiles[idx] = true;
    }

    pub fn get_tile(&self, x: i32, y: i32) -> TileType {
        self.tiles[self.xy_idx(x, y)]
    }

    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y * self.width + x) as usize
    }

    fn apply_room_to_map(&mut self, room: &Rect) {
        for y in room.y1 + 1..=room.y2 {
            for x in room.x1 + 1..=room.x2 {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = TileType::Floor;
            }
        }
    }
    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < (self.width * self.height) as usize {
                self.tiles[idx] = TileType::Floor;
            }
        }
    }
    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < (self.width * self.height) as usize {
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn is_exit_valid(&self, x: i32, y: i32) -> bool {
        if x < 1 || x > self.width - 1 || y < 1 || y > self.height - 1 {
            return false;
        }
        let idx = self.xy_idx(x, y);
        !self.blocked_tiles[idx]
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx as usize] == TileType::Wall
    }

    fn get_available_exits(&self, _idx: usize) -> Vec<(usize, f32)> {
        let mut exits = Vec::<(usize, f32)>::new();
        let x = _idx as i32 % self.width;
        let y = _idx as i32 / self.width;
        let w = self.width as usize;

        // cardinal directions
        if self.is_exit_valid(x - 1, y) {
            exits.push((_idx - 1, 1.0))
        };
        if self.is_exit_valid(x + 1, y) {
            exits.push((_idx + 1, 1.0))
        };
        if self.is_exit_valid(x + 1, y) {
            exits.push((_idx - w, 1.0))
        };
        if self.is_exit_valid(x + 1, y) {
            exits.push((_idx + w, 1.0))
        };

        // diagonal directions
        if self.is_exit_valid(x-1, y-1) {
            exits.push(((_idx - w) - 1, 1.45));
        };
        if self.is_exit_valid(x+1, y-1) {
            exits.push(((_idx - w) + 1, 1.45));
        };
        if self.is_exit_valid(x-1, y+1) {
            exits.push(((_idx + w) - 1, 1.45));
        };
        if self.is_exit_valid(x+1, y+1) {
            exits.push(((_idx + w) + 1, 1.45));
        };

        exits
    }

    fn get_pathing_distance(&self, _idx1: usize, _idx2: usize) -> f32 {
        let w = self.width as usize;
        let p1 = Point::new(_idx1 % w, _idx1 / w);
        let p2 = Point::new(_idx2 % w, _idx2 / w);
        rltk::DistanceAlg::Pythagoras.distance2d(p1, p2)
    }
}
