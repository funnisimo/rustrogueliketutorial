use bracket_lib::prelude as RLTK;
use specs::prelude::*;
use specs_derive::Component;
use std::cmp::{max, min};

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Renderable {
    glyph: RLTK::FontCharType,
    fg: RLTK::RGB,
    bg: RLTK::RGB,
}

#[derive(Component)]
struct LeftMover {}

#[derive(Component, Debug)]
struct Player {}

struct State {
    ecs: World,
}

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        pos.x = min(79, max(0, pos.x + delta_x));
        pos.y = min(49, max(0, pos.y + delta_y));
    }
}

fn player_input(gs: &mut State, ctx: &mut RLTK::BTerm) {
    // Player movement
    match ctx.key {
        None => {} // Nothing happened
        Some(key) => match key {
            RLTK::VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            RLTK::VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            RLTK::VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            RLTK::VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
            _ => {}
        },
    }
}

impl RLTK::GameState for State {
    fn tick(&mut self, ctx: &mut RLTK::BTerm) {
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

struct LeftWalker {}

impl<'a> System<'a> for LeftWalker {
    type SystemData = (ReadStorage<'a, LeftMover>, WriteStorage<'a, Position>);

    fn run(&mut self, (lefty, mut pos): Self::SystemData) {
        for (_lefty, pos) in (&lefty, &mut pos).join() {
            pos.x -= 1;
            if pos.x < 0 {
                pos.x = 79;
            }
        }
    }
}

impl State {
    fn run_systems(&mut self) {
        let mut lw = LeftWalker {};
        lw.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

fn main() -> RLTK::BError {
    let context = RLTK::BTermBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    let mut gs = State { ecs: World::new() };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<LeftMover>();
    gs.ecs.register::<Player>();

    gs.ecs
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: RLTK::to_cp437('@'),
            fg: RLTK::RGB::named(RLTK::YELLOW),
            bg: RLTK::RGB::named(RLTK::BLACK),
        })
        .with(Player {})
        .build();

    for i in 0..10 {
        gs.ecs
            .create_entity()
            .with(Position { x: i * 7, y: 20 })
            .with(Renderable {
                glyph: RLTK::to_cp437('☺'),
                fg: RLTK::RGB::named(RLTK::RED),
                bg: RLTK::RGB::named(RLTK::BLACK),
            })
            .with(LeftMover {})
            .build();
    }

    RLTK::main_loop(context, gs)
}
