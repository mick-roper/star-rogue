use rltk::{Point, Rltk, VirtualKeyCode};
use specs::prelude::*;
use specs_derive::Component;
use std::cmp::{max, min};

use super::components::{Position, ViewShed};
use super::{Map, RunState, State, TileType};

#[derive(Component, Debug)]
pub struct Player {}

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<ViewShed>();
    let mut player_pos = ecs.write_resource::<Point>();
    let map = ecs.fetch::<Map>();

    for (_player, pos, viewshed) in (&mut players, &mut positions, &mut viewsheds).join() {
        if !map.tile_is_blocked(pos.x + delta_x, pos.y + delta_y) {
            let (width, height) = map.get_dimensions();
            pos.x = min(width - 1, max(0, pos.x + delta_x));
            pos.y = min(height - 1, max(0, pos.y + delta_y));

            player_pos.x = pos.x;
            player_pos.y = pos.y;

            viewshed.dirty = true;
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
    match ctx.key {
        None => return RunState::Paused,
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
            _ => return RunState::Paused,
        },
    }

    RunState::Running
}
