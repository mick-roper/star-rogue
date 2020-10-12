use rltk::{Console, GameState, Rltk, RGB};
use specs::prelude::*;

mod components;
use components::*;

mod systems;
use systems::*;

mod player;
use player::{Player, player_input};

mod map;
use map::*;

pub struct State {
    ecs: World,
    map: Map,
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

        draw_map(&self.map, ctx);

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

    let mut gs = State {
        ecs: World::new(),
        map: Map::new(80, 50),
    };

    configure_state(&mut gs);

    rltk::main_loop(context, gs);
}

fn configure_state(gamestate: &mut State) {
    // register components
    gamestate.ecs.register::<Position>();
    gamestate.ecs.register::<Renderable>();
    gamestate.ecs.register::<LeftMover>();
    gamestate.ecs.register::<Player>();

    // create the player
    gamestate.ecs.create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            foreground: RGB::named(rltk::YELLOW),
            background: RGB::named(rltk::BLACK),
        })
        .with(Player{})
        .build();

    // create some other entities
    for i in 1..10 {
        gamestate.ecs.create_entity()
            .with(Position{
                x: i*7,
                y: 20,
            })
            .with(Renderable{
                glyph: rltk::to_cp437('☺'),
                foreground: RGB::named(rltk::RED),
                background: RGB::named(rltk::BLACK),
            })
            .with(LeftMover {})
            .build();
    }
}

fn draw_map(map: &Map, ctx: &mut Rltk) {
    let (width, height) = map.get_size();

    let wall: u8 = rltk::to_cp437('#');
    let path: u8 = rltk::to_cp437('.');

    for x in 0..width {
        for y in 0..height {
            let tile = map.get_tile(x, y);
            let glyph = match tile {
                Tile::Floor => { path },
                Tile::Wall => { wall },
            };

            ctx.set(x, y, RGB::named(rltk::BLUE), RGB::named(rltk::BLACK), glyph);
        }
    }
}
