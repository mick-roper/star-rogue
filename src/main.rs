use rltk::{Console, GameState, Rltk, RGB, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{min, max};

mod components;
use components::*;

struct State {
    ecs: World
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
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
        ecs: World::new()
    };

    configure_state(&mut gs);

    rltk::main_loop(context, gs);
}

fn configure_state(gamestate: &mut State) {
    // register components
    gamestate.ecs.register::<Position>();
    gamestate.ecs.register::<Renderable>();

    // create the player
    gamestate.ecs.create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            foreground: RGB::named(rltk::YELLOW),
            background: RGB::named(rltk::BLACK),
        })
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
            .build();
    }
}
