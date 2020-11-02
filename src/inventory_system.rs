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
                        ReadStorage<'a, AreaOfEffect>,
                        WriteStorage<'a, Confusion>,
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
            aoe,
            mut confused,
            map
        ) = data;

        for (entity, use_item) in (&entities, &use_item).join() {
            // TARGETING
            let mut targets: Vec<Entity> = Vec::new();
            match use_item.target {
                None => { targets.push(*player_entity); }
                Some(target) => {
                    let area_effect = aoe.get(use_item.item);
                    match area_effect {
                        None => {
                            for mob in map.get_tile_content(target.x, target.y).iter () {
                                targets.push(*mob);
                            }
                        }
                        Some(area_effect) => {
                            let mut blast_tiles = rltk::field_of_view(target, area_effect.radius, &*map);
                            let (map_width, map_height) = map.get_dimensions();
                            blast_tiles.retain(|p| p.x > 0 && p.x < map_width-1 && p.y > 0 && p.y < map_height -1);
                            for tile in blast_tiles.iter() {
                                for mob in map.get_tile_content(tile.x, tile.y).iter() {
                                    targets.push(*mob);
                                }
                            }
                        }
                    }
                }
            }

            // HEALING
            let item_heals = healing.get(use_item.item);
            match item_heals {
                None => {}
                Some(healer) => {
                    for target in targets.iter() {
                        let stats = combat_stats.get_mut(*target);
                        if let Some(stats) = stats {
                            stats.current_hp = i32::min(stats.max_hp, stats.current_hp + healer.heal_amount);
                            if entity == *player_entity {
                                gamelog.entries.push(format!("You drink the {}, regaining {} hp", names.get(use_item.item).unwrap().name, healer.heal_amount));
                            }
                        }
                    }
                }
            }

            // DAMAGE
            let item_damages = inflict_damage.get(use_item.item);
            match item_damages {
                None => {}
                Some(damage) => {
                    for mob in targets.iter() {
                        SufferDamage::new_damage(&mut suffer_damage, *mob, damage.damage);
                        if entity == *player_entity {
                            let mob_name = names.get(*mob).unwrap();
                            let item_name = names.get(use_item.item).unwrap();
                            gamelog.entries.push(format!("You use {} on {}, inflicting {} hp of damage.", item_name.name, mob_name.name, damage.damage));
                        }
                    }
                }
            }

            // CONFUSION
            let mut add_confusion = Vec::new();
            {
                let causes_confusion = confused.get(use_item.item);
                match causes_confusion {
                    None => {}
                    Some(confusion) => {
                        for mob in targets.iter() {
                            add_confusion.push((*mob, confusion.turns));
                            if entity == *player_entity {
                                let mob_name = names.get(*mob).unwrap();
                                let item_name = names.get(use_item.item).unwrap();
                                gamelog.entries.push(format!("You use {} on {}, confusing them.", item_name.name, mob_name.name));
                            }
                        }
                    }
                }
            }
            for mob in add_confusion.iter() {
                confused.insert(mob.0, Confusion{ turns: mob.1 }).expect("Unable to insert status");
            }

            // CLEANUP
            let consumable = consumables.get(use_item.item);
            match consumable {
                None => {}
                Some(_) => {
                    entities.delete(use_item.item).expect("Delete failed!")
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