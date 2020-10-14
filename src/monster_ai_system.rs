use specs::prelude::*;
use super::{ViewShed, Position, Map, Monster};
use rltk::{field_of_view, Point, console};

pub struct MonsterAI {}

impl <'a> System<'a> for MonsterAI {
    type SystemData = ( ReadStorage<'a, ViewShed>,
                        ReadStorage<'a, Position>,
                        ReadStorage<'a, Monster>);
    
    fn run(&mut self, data: Self::System) {
        let (viewshed, pos, monster) = data;

        for (viewshed, pos, _monster) in (&viewshed, &pos, &monster).join() {
            console::log("monster considers its own existence");
        }
    }
}