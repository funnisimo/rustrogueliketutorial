use super::{Map, Player, Position, State, TileType, Viewshed};
use bracket_lib::prelude as RLTK;
use specs::prelude::*;
use std::cmp::{max, min};

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let map = ecs.fetch::<Map>();

    for (_player, pos, viewshed) in (&mut players, &mut positions, &mut viewsheds).join() {
        let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);
        if map.tiles[destination_idx] != TileType::Wall {
            pos.x = min(79, max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));

            viewshed.dirty = true;
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut RLTK::BTerm) {
    // Player movement
    match ctx.key {
        None => {} // Nothing happened
        Some(key) => match key {
            RLTK::VirtualKeyCode::Left
            | RLTK::VirtualKeyCode::Numpad4
            | RLTK::VirtualKeyCode::H => try_move_player(-1, 0, &mut gs.ecs),

            RLTK::VirtualKeyCode::Right
            | RLTK::VirtualKeyCode::Numpad6
            | RLTK::VirtualKeyCode::L => try_move_player(1, 0, &mut gs.ecs),

            RLTK::VirtualKeyCode::Up | RLTK::VirtualKeyCode::Numpad8 | RLTK::VirtualKeyCode::K => {
                try_move_player(0, -1, &mut gs.ecs)
            }

            RLTK::VirtualKeyCode::Down
            | RLTK::VirtualKeyCode::Numpad2
            | RLTK::VirtualKeyCode::J => try_move_player(0, 1, &mut gs.ecs),

            _ => {}
        },
    }
}
