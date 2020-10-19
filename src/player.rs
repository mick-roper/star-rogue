use rltk::{Point, Rltk, VirtualKeyCode, console};
use specs::prelude::*;
use specs_derive::Component;
use std::cmp::{max, min};

use super::components::{Position, ViewShed, CombatStats};
use super::{Map, RunState, State};

#[derive(Component, Debug)]
pub struct Player {}

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<ViewShed>();
    let mut player_pos = ecs.write_resource::<Point>();
    let map = ecs.fetch::<Map>();
    let combat_stats = ecs.read_storage::<CombatStats>();

    for (_player, pos, viewshed) in (&mut players, &mut positions, &mut viewsheds).join() {
        let (width, height) = map.get_dimensions();
        let x = min(width - 1, max(0, pos.x + delta_x));
        let y = min(height - 1, max(0, pos.y + delta_y));
        for potential_target in map.get_tile_content(x, y) {
            let target = combat_stats.get(*potential_target);
            match target {
                None => {}
                Some(t) => {
                    console::log(&format!("From hells heart I stab at thee!"));
                    return;
                }
            }
        }

        if !map.tile_is_blocked(x, y) {
            pos.x = x;
            pos.y = y;
            player_pos.x = x;
            player_pos.y = y;

            viewshed.dirty = true;
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
    match ctx.key {
        None => return RunState::Paused,
        Some(key) => match key {
            VirtualKeyCode::Left |
            VirtualKeyCode::Numpad4 |
            VirtualKeyCode::H => try_move_player(-1, 0, &mut gs.ecs),

            VirtualKeyCode::Right |
            VirtualKeyCode::Numpad6 |
            VirtualKeyCode::L => try_move_player(1, 0, &mut gs.ecs),

            VirtualKeyCode::Up |
            VirtualKeyCode::Numpad8 |
            VirtualKeyCode::K => try_move_player(0, -1, &mut gs.ecs),

            VirtualKeyCode::Down |
            VirtualKeyCode::Numpad2 |
            VirtualKeyCode::J => try_move_player(0, 1, &mut gs.ecs),

            // diagonals
            VirtualKeyCode::Numpad9 |
            VirtualKeyCode::Y => try_move_player(1, -1, &mut gs.ecs),

            VirtualKeyCode::Numpad7 |
            VirtualKeyCode::U => try_move_player(-1, -1, &mut gs.ecs),

            VirtualKeyCode::Numpad3 |
            VirtualKeyCode::N => try_move_player(1, 1, &mut gs.ecs),

            VirtualKeyCode::Numpad1 |
            VirtualKeyCode::B => try_move_player(-1, 1, &mut gs.ecs),

            _ => return RunState::Paused,
        },
    }

    RunState::Running
}
