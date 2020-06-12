use rltk::{Rltk, Console, RGB, RandomNumberGenerator};
use super::{Rect};
use std::cmp::{max, min};

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall, Floor
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

pub fn new_map() -> (Vec<TileType>, Vec<Rect>) {
    let mut map = vec![TileType::Wall; 80*50];
    let mut rooms: Vec<Rect> = Vec::new();

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
        for other_room in rooms.iter() {
            if new_room.intersect(other_room) { ok = false }
        }

        if ok {
            add_room_to_map(&new_room, &mut map);

            if !rooms.is_empty() {
                let (new_x, new_y) = new_room.centre();
                let (prev_x, prev_y) = rooms[rooms.len() - 1].centre();
                if rng.range(1, 2) == 1 {
                    add_horizontal_tunnel(&mut map, prev_x, new_x, prev_y);
                    add_vertical_tunnel(&mut map, prev_y, new_y, new_x);
                } else {
                    add_vertical_tunnel(&mut map, prev_y, new_y, new_x);
                    add_horizontal_tunnel(&mut map, prev_x, new_x, prev_y);
                }
            }

            rooms.push(new_room);
        }
    }

    (map, rooms)
}

pub fn draw_map(map: &[TileType], ctx: &mut Rltk) {
    let mut y = 0;
    let mut x = 0;

    let floor_colour = RGB::from_f32(0.5, 0.5, 0.5);
    let wall_colour = RGB::from_f32(0.5, 0.5, 0.5);
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

fn add_room_to_map(room: &Rect, map: &mut [TileType]) {
    for y in room.y1 +1 ..= room.y2 {
        for x in room.x1 + 1 ..= room.x2 {
            map[xy_idx(x, y)] = TileType::Floor;
        }
    }
}

fn add_horizontal_tunnel(map: &mut [TileType], x1: i32, x2: i32, y: i32) {
    for x in min(x1, x2) ..= max(x1, x2) {
        let idx = xy_idx(x, y);
        if idx > 0 && idx < 80*50 {
            map[idx as usize] = TileType::Floor;
        }
    }
}

fn add_vertical_tunnel(map: &mut [TileType], y1: i32, y2: i32, x: i32) {
    for y in min(y1, y2) ..= max(y1, y2) {
        let idx = xy_idx(x, y);
        if idx > 0 && idx < 80*50 {
            map[idx as usize] = TileType::Floor;
        }
    }
}