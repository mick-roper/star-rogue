use specs::prelude::*;
use super::{ViewShed, Monster, Name};
use rltk::{console, Point};

pub struct MonsterAI {}

impl <'a> System<'a> for MonsterAI {
    type SystemData = ( ReadStorage<'a, ViewShed>,
                        ReadExpect<'a, Point>,
                        ReadStorage<'a, Monster>,
                        ReadStorage<'a, Name>);
    
    fn run(&mut self, data: Self::SystemData) {
        let (viewshed, player_pos, monster, name) = data;

        for (viewshed, _monster, name) in (&viewshed, &monster, &name).join() {
            if viewshed.visible_tiles.contains(&*player_pos) {
                console::log(format!("{} shouts insults", name.name));
            }
        }
    }
}