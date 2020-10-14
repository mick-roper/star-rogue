use rltk::{Console, GameState, Rltk, RGB};
use specs::prelude::*;

mod components;
use components::*;

mod player;
use player::{player_input, Player};

mod rect;
use rect::*;

mod map;
use map::*;

mod visibility_system;
use visibility_system::*;

pub struct State {
    ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);

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

        draw_map(&self.ecs, ctx);

        // draw objects
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();

        for (pos, render) in (&positions, &renderables).join() {
            if map.tile_is_visible(pos.x, pos.y) {
                ctx.set(
                    pos.x,
                    pos.y,
                    render.foreground,
                    render.background,
                    render.glyph,
                )
            }
        }
    }
}

fn main() {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50().with_title("Star Rogue").build();

    let state = build_state();

    rltk::main_loop(context, state);
}

fn build_state() -> State {
    let map = Map::new(80, 50);
    let mut gs = State { ecs: World::new() };
    let mut rng = rltk::RandomNumberGenerator::new();
    // register components
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<LeftMover>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<ViewShed>();

    // create the player
    let (player_x, player_y) = map.get_room(0).centre();
    gs.ecs
        .create_entity()
        .with(Position {
            x: player_x,
            y: player_y,
        })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            foreground: RGB::named(rltk::YELLOW),
            background: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .with(ViewShed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .build();

    // create some enemies
    for i in 1..map.get_room_count() {
        let (x, y) = map.get_room(i).centre();
        let glyph = match rng.roll_dice(1, 2) {
            1 => { rltk::to_cp437('g') }
            _ => { rltk::to_cp437('o') }
        };

        gs.ecs
            .create_entity()
            .with(Position { x, y })
            .with(Renderable {
                glyph: glyph,
                foreground: RGB::named(rltk::RED),
                background: RGB::named(rltk::BLACK),
            })
            .with(ViewShed {
                visible_tiles: Vec::new(),
                range: 8,
                dirty: true,
            })
            .with(Monster {})
            .build();
    }

    gs.ecs.insert(map);

    gs
}

fn draw_map(ecs: &World, ctx: &mut Rltk) {
    let map = ecs.fetch::<Map>();

    let wall: u8 = rltk::to_cp437('#');
    let path: u8 = rltk::to_cp437('.');
    let black = RGB::named(rltk::BLACK);

    let (width, height) = map.get_dimensions();

    for x in 0..width {
        for y in 0..height {
            if map.tile_is_revealed(x, y) {
                let tile = map.get_tile(x, y);
                let glyph;
                let mut fg;
                match tile {
                    TileType::Floor => {
                        glyph = path;
                        fg = RGB::from_f32(0.0, 0.5, 0.5);
                    }
                    TileType::Wall => {
                        glyph = wall;
                        fg = RGB::from_f32(0., 1.0, 0.);
                    }
                };

                if !map.tile_is_visible(x, y) {
                    fg = fg.to_greyscale()
                }
                ctx.set(x, y, fg, black, glyph);
            }
        }
    }
}
