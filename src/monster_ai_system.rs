use super::*;
use rltk::{Point};

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadExpect<'a, Point>,
        ReadExpect<'a, Entity>,
        Entities<'a>,
        WriteStorage<'a, ViewShed>,
        ReadStorage<'a, Monster>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, WantsToMelee>,
        ReadExpect<'a, RunState>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut map,
            player_pos, 
            player_entity,
            entities,
            mut viewshed,
            monsters,
            mut position,
            mut wants_to_melee,
            runstate
        ) = data;

        if *runstate != RunState::MonsterTurn { return; }

        for (entity, mut viewshed, _monster, mut pos) in (&entities, &mut viewshed, &monsters, &mut position).join() {            
            let distance = rltk::DistanceAlg::Pythagoras.distance2d(Point::new(pos.x, pos.y), *player_pos);
            if distance < 1.5 {
                wants_to_melee.insert(entity, WantsToMelee { target: *player_entity }).expect("unable to insert an attack");
            } else if viewshed.visible_tiles.contains(&*player_pos) {
                let path = rltk::a_star_search(
                    map.xy_idx(pos.x, pos.y),
                    map.xy_idx(player_pos.x, player_pos.y),
                    &mut *map
                );

                if path.success && path.steps.len() > 1 {
                    let (width, _) = map.get_dimensions();
                    map.set_tile_as_unblocked(pos.x, pos.y);
                    pos.x = path.steps[1] as i32 % width;
                    pos.y = path.steps[1] as i32 / width;
                    map.set_tile_as_blocked(pos.x, pos.y);
                    viewshed.dirty = true;
                }
            }
        }
    }
}
