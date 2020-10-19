use specs::prelude::*;
use rltk::{console};
use super::*;

pub struct DamageSystem {}

impl <'a> System<'a> for DamageSystem {
    type SystemData = ( WriteStorage<'a, CombatStats>,
                        WriteStorage<'a, SufferDamage>);

    fn run (&mut self, data: Self::SystemData) {
        let (mut stats, mut damage) = data;

        for (mut stats, damage) in (&mut stats, &damage).join() {
            stats.current_hp -= damage.amount.iter().sum::<i32>();
        }

        damage.clear();
    }
}

pub fn delete_the_dead(ecs: &mut World) {
    let mut dead: Vec<Entity> = Vec::new();

    // use a scope to make the borrow-checker happy
    {
        let combat_stats = ecs.read_storage::<CombatStats>();
        let entities = ecs.entities();
        let players = ecs.read_storage::<Player>();

        for (entity, stats) in (&entities, &combat_stats).join() {
            if stats.current_hp < 1 { 
                let player = players.get(entity);
                match player {
                    None => dead.push(entity),
                    Some(_) => console::log("You are dead!")
                }
            }
        }
    }

    for victim in dead {
        ecs.delete_entity(victim).expect("Unable to delete");
    }
}