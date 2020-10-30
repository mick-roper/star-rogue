use specs::prelude::*;
use super::*;

pub struct PotionUseSystem {}

impl<'a> System<'a> for PotionUseSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = ( ReadExpect<'a, Entity>,
                        WriteExpect<'a, GameLog>,
                        Entities<'a>,
                        WriteStorage<'a, WantsToDrinkPotion>,
                        ReadStorage<'a, Name>,
                        ReadStorage<'a, Potion>,
                        WriteStorage<'a, CombatStats>
                    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            player_entity, 
            mut gamelog, 
            entities, 
            wants_drink, 
            names, 
            potions, 
            mut combat_stats
        ) = data;

        for (entity, drink, stats) in (&entities, &wants_drink, &mut combat_stats).join() {
            let potion = potions.get(drink.potion);
            match potion {
                None => {}
                Some(potion) => {
                    stats.current_hp = i32::min(stats.max_hp, stats.current_hp + potion.heal_amount);
                    if entity == *player_entity {
                        gamelog.entries.push(format!("You dring the {}, regaining {} hp", names.get(drink.potion).unwrap().name, potion.heal_amount));
                    }
                    entities.delete(drink.potion).expect("Delete failed");
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