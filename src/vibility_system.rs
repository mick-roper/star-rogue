use specs::prelude::*;
use super::*;
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

        let (map_width, map_height) = map.get_dimensions();

        for (ent, viewshed, pos) in (&entities, &mut viewshed, &pos).join() {
            viewshed.visible_tiles.clear();
            viewshed.visible_tiles = field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map);
            viewshed.visible_tiles.retain(|p| p.x > 0 && p.x < map_width - 1 && p.y > 0 && p.y < map_height -1);

            // if this is the player, reveal what they can see
            let p: Option<&Player> = player.get(ent);
            if let Some(_p) = p {
                map.clear_visible_tiles();
                for vis in viewshed.visible_tiles.iter() {
                    map.reveal_tile(vis.x, vis.y);
                    map.mark_tile_as_visible(vis.x, vis.y);
                }
            }
        }
    }
}