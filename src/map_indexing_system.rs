use specs::prelude::*;
use super::{Map, Position, BlocksTile};

pub struct MapIndexingSystem {}

impl <'a> System<'a> for MapIndexingSystem {
    type SystemData = ( WriteExpect<'a, Map>,
                        ReadStorage<'a, Position>,
                        ReadStorage<'a, BlocksTile>);
    
    fn run(&mut self, data: Self::SystemData) {
        let (mut map, position, blockers) = data;

        map.update_blocked_tiles();
        for (position, _blocks) in (&position, &blockers).join() {
            map.set_tile_as_blocked(position.x, position.y);
        }
    }
}