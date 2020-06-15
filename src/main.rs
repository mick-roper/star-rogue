use rltk::{Rltk, GameState, Console, RGB, Point};
use specs::prelude::*;

mod components;
pub use components::*;

mod shapes;
pub use shapes::*;

mod map;
pub use map::*;

mod player;
pub use player::*;

mod visibility_system;
pub use visibility_system::VisibilitySystem;

pub struct State {
    ecs: World
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        player_input(self, ctx);

        self.run_systems();

        let map = self.ecs.fetch::<Map>();
        draw_map(&self.ecs, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem{};
        vis.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

fn main() {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Star Rogue")
        .build();
    
    let mut gs = State {
        ecs: World::new()
    };

    let map = new_map(80, 50);
    let (player_x, player_y) = map.rooms[0].centre();

    gs.ecs.insert(map);

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();

    gs.ecs
        .create_entity()
        .with(Position { x: player_x, y: player_y })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player{})
        .with(Viewshed{ visible_tiles: Vec::new(), range: 8, dirty: true })
        .build();

    rltk::main_loop(context, gs);
}

fn draw_map(ecs: &World, ctx: &mut Rltk) {
    let map = ecs.fetch::<Map>();

    let mut y = 0;
    let mut x = 0;

    let floor_colour = RGB::from_f32(0.5, 0.5, 0.5);
    let wall_colour = RGB::from_f32(0.0, 1.0, 0.0);
    let black = RGB::from_f32(0., 0., 0.);
    let dot = rltk::to_cp437('.');
    let hash = rltk::to_cp437('#');

    for (idx, tile) in map.tiles.iter().enumerate() {
        if map.revealed_tiles[idx] {
            match tile {
                TileType::Floor => {
                    ctx.set(x, y, floor_colour, black, dot);
                },
                TileType::Wall => {
                    ctx.set(x, y, wall_colour, black, hash);
                }
            }
        }

        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}