use specs::prelude::*;
use super::*;
use rltk::{field_of_view, Point, console};

pub struct MapIndexingSystem {}

impl <'a> System<'a> for MapIndexingSystem {
    type SystemData = ( WriteExpect<'a, Map>,
                        ReadStorage<'a, Position>,
                        ReadStorage<'a, BlocksTile>,
                        Entities<'a>,);
    
    fn run(&mut self, data: Self::SystemData) {
        let (mut map, position, blockers, entities) = data;

        map.update_blocked_tiles();
        map.clear_content_index();

        for (entity, pos) in (&entities, &position).join() {
            let _p: Option<&BlocksTile> = blockers.get(entity);
            if let Some(_p) = _p {
                map.set_tile_as_blocked(pos.x, pos.y);
            }

            map.add_tile_content(pos.x, pos.y, entity);
        }
    }
}