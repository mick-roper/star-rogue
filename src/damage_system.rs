use specs::prelude::*;
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