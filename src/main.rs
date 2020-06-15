use rltk::{RGB, Point};
use specs::prelude::*;

mod components;
pub use components::*;

mod shapes;
pub use shapes::*;

mod state;
pub use state::*;

mod map;
pub use map::*;

mod player;
pub use player::*;

mod visibility_system;
pub use visibility_system::VisibilitySystem;

mod monster_ai_system;
pub use monster_ai_system::MonsterAI;

fn main() {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Star Rogue")
        .build();
    
    let mut gs = State {
        ecs: World::new(),
        runstate: RunState::Running
    };

    let map = new_map(80, 50);
    let (player_x, player_y) = map.rooms[0].centre();

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Monster>();

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

    let mut rng = rltk::RandomNumberGenerator::new();

    for room in map.rooms.iter().skip(1) {
        let (x, y) = room.centre();
        let glyph: u8;
        let roll = rng.roll_dice(1, 2);
        match roll {
            1 => { glyph = rltk::to_cp437('g') },
            _ => { glyph = rltk::to_cp437('o') }
        }

        gs.ecs.create_entity()
            .with(Position{ x, y })
            .with(Renderable { 
                glyph, 
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK)
            })
            .with(Viewshed { visible_tiles: Vec::new(), range: 8, dirty: true })
            .with(Monster{})
            .build();
    }

    gs.ecs.insert(Point::new(player_x, player_y));
    gs.ecs.insert(map);

    rltk::main_loop(context, gs);
}