use rltk::{RGB, RandomNumberGenerator};
use specs::prelude::*;
use super::*;

pub fn player(ecs: &mut World, player_x: i32, player_y: i32) -> Entity {
    ecs
        .create_entity()
        .with(Position{ x: player_x, y: player_y })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            foreground: RGB::named(rltk::YELLOW),
            background: RGB::named(rltk::BLACK),
        })
        .with(Player{})
        .with(ViewShed{ 
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .with(Name{ name: "Player".to_string() })
        .with(CombatStats::new(30, 2, 5))
        .build()
}

pub fn random_monster(ecs: &mut World, x: i32, y: i32) {
    let roll: i32;
    {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        roll = rng.roll_dice(1, 2);
    }
    match roll {
        1 => { orc(ecs, x, y) }
        _ => { goblin(ecs, x, y) }
    }
}

fn orc(ecs: &mut World, x: i32, y: i32) { monster(ecs, x, y, rltk::to_cp437('o'), "Orc"); }
fn goblin(ecs: &mut World, x: i32, y: i32) { monster(ecs, x, y, rltk::to_cp437('g'), "Goblin"); }

fn monster<S: ToString>(ecs: &mut World, x: i32, y: i32, glyph: u8, name: S) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph,
            foreground: RGB::named(rltk::RED),
            background: RGB::named(rltk::BLACK),
        })
        .with(ViewShed{ 
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .with(Monster {})
        .with(Name { name: name.to_string() })
        .with(BlocksTile {})
        .with(CombatStats::new(16, 1, 4))
        .build();
}