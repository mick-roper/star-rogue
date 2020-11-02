use specs::prelude::*;
use super::*;

pub struct ItemUseSystem {}

impl<'a> System<'a> for ItemUseSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = ( ReadExpect<'a, Entity>,
                        WriteExpect<'a, GameLog>,
                        Entities<'a>,
                        WriteStorage<'a, WantsToUseItem>,
                        ReadStorage<'a, Name>,
                        ReadStorage<'a, Consumable>,
                        ReadStorage<'a, ProvidesHealing>,
                        WriteStorage<'a, CombatStats>
                    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            player_entity, 
            mut gamelog, 
            entities, 
            use_item, 
            names, 
            consumables,
            healing,
            mut combat_stats
        ) = data;

        for (entity, want_to_use, stats) in (&entities, &use_item, &mut combat_stats).join() {
            let item_heals = healing.get(want_to_use.item);
            match item_heals {
                None => {}
                Some(healer) => {
                    stats.current_hp = i32::min(stats.max_hp, stats.current_hp + healer.heal_amount);
                    if entity == *player_entity {
                        gamelog.entries.push(format!("You drink the {}, regaining {} hp", names.get(want_to_use.item).unwrap().name, healer.heal_amount));
                    }
                }
            }

            let consumable = consumables.get(want_to_use.item);
            match consumable {
                None => {}
                Some(_) => {
                    entities.delete(want_to_use.item).expect("Delete failed!")
                }
            }
        }
    }
}

pub struct ItemDropSystem {}

impl<'a> System<'a> for ItemDropSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = ( ReadExpect<'a, Entity>,
                        WriteExpect<'a, GameLog>,
                        Entities<'a>,
                        WriteStorage<'a, WantsToDropItem>,
                        ReadStorage<'a, Name>,
                        WriteStorage<'a, Position>,
                        WriteStorage<'a, InBackPack>
                    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            player_entity, 
            mut gamelog,
            entites, 
            mut wants_drop,
            names,
            mut positions,
            mut backpack
        ) = data;

        for (entity, to_drop) in (&entites, &wants_drop).join() {
            let mut dropper_pos: Position = Position{x: 0, y: 0};
            {
                let dropped_pos = positions.get(entity).unwrap();
                dropper_pos.x = dropped_pos.x;
                dropper_pos.y = dropped_pos.y;
            }

            positions.insert(to_drop.item, dropper_pos).expect("Unable to insert position");
            backpack.remove(to_drop.item);

            if entity == *player_entity {
                gamelog.entries.push(format!("You drop the {}", names.get(to_drop.item).unwrap().name));
            }
        }

        wants_drop.clear();
    }
}