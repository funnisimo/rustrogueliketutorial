use bracket_lib::prelude as RLTK;
use specs::prelude::*;

mod components;
pub use components::*;
mod map;
pub use map::*;
mod player;
use player::*;

pub struct State {
    pub ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        self.ecs.maintain();
    }
}

impl RLTK::GameState for State {
    fn tick(&mut self, ctx: &mut RLTK::BTerm) {
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();

        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

fn main() -> RLTK::BError {
    let context = RLTK::BTermBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    let mut gs = State { ecs: World::new() };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();

    let (rooms, map) = new_map_rooms_and_corridors();
    gs.ecs.insert(map);
    let player_pos = rooms[0].center();

    gs.ecs
        .create_entity()
        .with(Position {
            x: player_pos.x,
            y: player_pos.y,
        })
        .with(Renderable {
            glyph: RLTK::to_cp437('@'),
            fg: RLTK::RGB::named(RLTK::YELLOW),
            bg: RLTK::RGB::named(RLTK::BLACK),
        })
        .with(Player {})
        .build();

    RLTK::main_loop(context, gs)
}
