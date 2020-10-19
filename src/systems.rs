use specs::prelude::*;
use super::*;
use rltk::{field_of_view, Point, console};

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

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadExpect<'a, Point>,
        WriteStorage<'a, ViewShed>,
        ReadStorage<'a, Monster>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, Position>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (mut map, player_pos, mut viewshed, monster, name, mut position) = data;

        for (mut viewshed, _monster, name, mut pos) in
            (&mut viewshed, &monster, &name, &mut position).join()
        {
            if viewshed.visible_tiles.contains(&*player_pos) {
                let distance =
                    rltk::DistanceAlg::Pythagoras.distance2d(Point::new(pos.x, pos.y), *player_pos);
                if distance < 1.5 {
                    console::log(&format!("{} shouts insults", name.name));
                    return;
                }

                let path = rltk::a_star_search(
                    map.xy_idx(pos.x, pos.y) as i32,
                    map.xy_idx(player_pos.x, player_pos.y) as i32,
                    &mut *map,
                );

                if path.success && path.steps.len() > 1 {
                    let (width, _) = map.get_dimensions();
                    pos.x = path.steps[1] as i32 % width;
                    pos.y = path.steps[1] as i32 / width;
                    viewshed.dirty = true;
                }
            }
        }
    }
}

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

struct MeleeCombatSystem {}

impl<'a> System<'a> for MeleeCombatSystem {
    type SystemData = ( Entities<'a>,
                        WriteStorage<'a, WantsToMelee>,
                        ReadStorage<'a, Name>,
                        ReadStorage<'a, CombatStats>,
                        WriteStorage<'a, SufferDamage>
                      );

    fn run(&mut self, data : Self::SystemData) {
        let (entities, mut wants_melee, names, combat_stats, mut inflict_damage) = data;

        for (_entity, wants_melee, name, stats) in (&entities, &wants_melee, &names, &combat_stats).join() {
            if stats.hp > 0 {
                let target_stats = combat_stats.get(wants_melee.target).unwrap();
                if target_stats.hp > 0 {
                    let target_name = names.get(wants_melee.target).unwrap();

                    let damage = i32::max(0, stats.power - target_stats.defense);

                    if damage == 0 {
                        console::log(&format!("{} is unable to hurt {}", &name.name, &target_name.name));
                    } else {
                        console::log(&format!("{} hits {}, for {} hp.", &name.name, &target_name.name, damage));
                        SufferDamage::new_damage(&mut inflict_damage, wants_melee.target, damage);
                    }
                }
            }
        }

        wants_melee.clear();
    }
}