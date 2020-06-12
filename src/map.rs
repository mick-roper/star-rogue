use rltk::{Rltk, Console, RGB};

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall, Floor
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

pub fn new_map() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; 80*50];

    for x in 0..80 {
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, 49)] = TileType::Wall;
    }

    for y in 0..50 {
        map[xy_idx(0, y)] = TileType::Wall;        
        map[xy_idx(79, y)] = TileType::Wall;
    }

    let mut rng = rltk::RandomNumberGenerator::new();

    for _i in 0..400 {
        let x = rng.roll_dice(1, 79);
        let y = rng.roll_dice(1, 49);
        let idx = xy_idx(x, y);
        if idx != xy_idx(40, 25) {
            map[idx] = TileType::Wall;
        }
    }

    map
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