use specs::prelude::*;
use super::{ViewShed, Position, Map, Player};
use rltk::{field_of_view, Point};

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (WriteExpect<'a, Map>,
        Entities<'a>,
        WriteStorage<'a, ViewShed>, 
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, entities, mut viewshed, pos, player) = data;

        for (ent, viewshed, pos) in (&entities, &mut viewshed, &pos).join() {
            viewshed.visible_tiles.clear();
            viewshed.visible_tiles = field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map);
            viewshed.visible_tiles.retain(|p| p.x > 0 && p.x < map.width - 1 && p.y > 0 && p.y < map.height -1);

            // if this is the player, reveal what they can see
            let p: Option<&Player> = player.get(ent);
            if let Some(p) = p {
                for vis in viewshed.visible_tiles.iter() {
                    map.reveal(vis.x, vis.y);
                }
            }
        }
    }
}