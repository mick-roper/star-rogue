use super::*;
use rltk::{RandomNumberGenerator, RGB};
use specs::saveload::MarkedBuilder;

const MAX_MONSTERS_PER_ROOM: i32 = 4;
const MAX_ITEMS_PER_ROOM: i32 = 2;

pub fn player(ecs: &mut World, player_x: i32, player_y: i32) -> Entity {
    ecs.create_entity()
        .with(Position {
            x: player_x,
            y: player_y,
        })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            foreground: RGB::named(rltk::YELLOW),
            background: RGB::named(rltk::BLACK),
            render_order: 0,
        })
        .with(Player {})
        .with(ViewShed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .with(Name {
            name: "Player".to_string(),
        })
        .with(CombatStats::new(30, 2, 5))
        .marked::<SimpleMarker<SerializeMe>>()
        .build()
}

pub fn spawn_room(ecs: &mut World, map_width: i32, room: &Rect) {
    let mut monster_spawn_points: Vec<i32> = Vec::new();
    let mut item_spawn_points: Vec<i32> = Vec::new();

    {
        // this scope keeps the borrow checker happy
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        let num_monsters = rng.roll_dice(1, MAX_MONSTERS_PER_ROOM + 2) - 3;
        let num_items = rng.roll_dice(1, MAX_ITEMS_PER_ROOM + 2) - 3;

        for _i in 0..num_monsters {
            let mut added = false;
            while !added {
                let x = room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1));
                let y = room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1));
                let idx = y * map_width + x;
                if !monster_spawn_points.contains(&idx) {
                    monster_spawn_points.push(idx);
                    added = true;
                }
            }
        }

        for _i in 0..num_items {
            let mut added = false;
            while !added {
                let x = room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1));
                let y = room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1));
                let idx = y * map_width + x;
                if !item_spawn_points.contains(&idx) {
                    item_spawn_points.push(idx);
                    added = true;
                }
            }
        }
    }

    for idx in monster_spawn_points.iter() {
        let x = *idx % map_width;
        let y = *idx / map_width;
        random_monster(ecs, x as i32, y as i32);
    }

    for idx in item_spawn_points.iter() {
        let x = *idx % map_width;
        let y = *idx / map_width;
        random_item(ecs, x as i32, y as i32);
    }
}

fn random_monster(ecs: &mut World, x: i32, y: i32) {
    let roll: i32;
    {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        roll = rng.roll_dice(1, 2);
    }
    match roll {
        1 => orc(ecs, x, y),
        _ => goblin(ecs, x, y),
    }
}

fn orc(ecs: &mut World, x: i32, y: i32) {
    monster(ecs, x, y, rltk::to_cp437('o'), "Orc");
}
fn goblin(ecs: &mut World, x: i32, y: i32) {
    monster(ecs, x, y, rltk::to_cp437('g'), "Goblin");
}

fn monster<S: ToString>(ecs: &mut World, x: i32, y: i32, glyph: u8, name: S) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph,
            foreground: RGB::named(rltk::RED),
            background: RGB::named(rltk::BLACK),
            render_order: 1,
        })
        .with(ViewShed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .with(Monster {})
        .with(Name {
            name: name.to_string(),
        })
        .with(BlocksTile {})
        .with(CombatStats::new(16, 1, 4))
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn random_item(ecs: &mut World, x: i32, y: i32) {
    let roll: i32;
    {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        roll = rng.roll_dice(1, 4);
    }
    match roll {
        1 => health_potion(ecs, x, y),
        2 => fireball_scroll(ecs, x, y),
        3 => confusion_scroll(ecs, x, y),
        _ => magic_missile_scroll(ecs, x, y),
    };
}

fn health_potion(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: rltk::to_cp437('i'),
            foreground: RGB::named(rltk::MAGENTA),
            background: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name {
            name: "Health Potion".to_string(),
        })
        .with(Item {})
        .with(Consumable {})
        .with(ProvidesHealing { heal_amount: 8 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn magic_missile_scroll(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: rltk::to_cp437(')'),
            foreground: RGB::named(rltk::CYAN),
            background: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name {
            name: "Scroll of Magic Missle".to_string(),
        })
        .with(Item {})
        .with(Ranged { range: 6 })
        .with(InflictsDamage { damage: 8 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn fireball_scroll(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: rltk::to_cp437(')'),
            foreground: RGB::named(rltk::ORANGE),
            background: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name {
            name: "Scroll of Fireball".to_string(),
        })
        .with(Item {})
        .with(Ranged { range: 6 })
        .with(InflictsDamage { damage: 20 })
        .with(AreaOfEffect { radius: 3 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn confusion_scroll(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: rltk::to_cp437(')'),
            foreground: RGB::named(rltk::PINK),
            background: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name {
            name: "Scroll of Confusion".to_string(),
        })
        .with(Item {})
        .with(Ranged { range: 6 })
        .with(Confusion { turns: 4 })
        .with(AreaOfEffect { radius: 3 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}
