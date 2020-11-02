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
                        ReadStorage<'a, InflictsDamage>,
                        WriteStorage<'a, CombatStats>,
                        WriteStorage<'a, SufferDamage>,
                        ReadExpect<'a, Map>,
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
            inflict_damage,
            mut combat_stats,
            mut suffer_damage,
            map
        ) = data;

        for (entity, use_item, stats) in (&entities, &use_item, &mut combat_stats).join() {
            let item_heals = healing.get(use_item.item);
            let mut used_item = false;
            match item_heals {
                None => {}
                Some(healer) => {
                    stats.current_hp = i32::min(stats.max_hp, stats.current_hp + healer.heal_amount);
                    if entity == *player_entity {
                        gamelog.entries.push(format!("You drink the {}, regaining {} hp", names.get(use_item.item).unwrap().name, healer.heal_amount));
                    }
                    used_item = true;
                }
            }

            let item_damages = inflict_damage.get(use_item.item);
            match item_damages {
                None => {}
                Some(damage) => {
                    let target_point = use_item.target.unwrap();
                    for mob in map.get_tile_content(target_point.x, target_point.y).iter() {
                        SufferDamage::new_damage(&mut suffer_damage, *mob, damage.damage);
                        if entity == *player_entity {
                            let mob_name = names.get(*mob).unwrap();
                            let item_name = names.get(use_item.item).unwrap();
                            gamelog.entries.push(format!("You use {} on {}, inflicting {} hp of damage.", item_name.name, mob_name.name, damage.damage));
                        }
                        used_item = true;
                    }
                }
            }

            let consumable = consumables.get(use_item.item);
            match consumable {
                None => {}
                Some(_) => {
                    if used_item {
                        entities.delete(use_item.item).expect("Delete failed!")
                    }
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