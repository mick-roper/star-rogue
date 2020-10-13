use rltk::{Console, GameState, Rltk, RGB};
use specs::prelude::*;

mod components;
use components::*;

mod systems;
use systems::*;

mod player;
use player::{Player, player_input};

mod rect;
use rect::*;

mod map;
use map::*;

pub struct State {
    ecs: World
}

impl State {
    fn run_systems(&mut self) {
        let mut lw = LeftWalker{};
        lw.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        // get input
        player_input(self, ctx);

        // update items
        self.run_systems();

        let map = self.ecs.fetch::<Map>();
        draw_map(&map, ctx);

        // draw objects
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.foreground, render.background, render.glyph)
        }
    }
}

fn main() {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50().with_title("Star Rogue").build();

    let state = build_state();

    rltk::main_loop(context,state);
}

fn build_state() -> State {
    let map = Map::new(80, 50);
    let mut gs = State {
        ecs: World::new()
    };

    let (player_x, player_y) = map.get_room(0).centre();

    gs.ecs.insert(map);
    // register components
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<LeftMover>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<ViewShed>();

    // create the player
    gs.ecs.create_entity()
        .with(Position { x: player_x, y: player_y })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            foreground: RGB::named(rltk::YELLOW),
            background: RGB::named(rltk::BLACK),
        })
        .with(Player{})
        .with(ViewShed{ visible_tiles: Vec::new(), range: 8 })
        .build();

    gs
}

fn draw_map(map: &Map, ctx: &mut Rltk) {
    let (width, height) = map.get_size();

    let wall: u8 = rltk::to_cp437('#');
    let path: u8 = rltk::to_cp437('.');

    for x in 0..width {
        for y in 0..height {
            let tile = map.get_tile(x, y);
            let glyph = match tile {
                TileType::Floor => { path },
                TileType::Wall => { wall },
            };

            ctx.set(x, y, RGB::named(rltk::BLUE), RGB::named(rltk::BLACK), glyph);
        }
    }
}
