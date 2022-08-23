use bracket_lib::prelude as RLTK;

struct State {}
impl RLTK::GameState for State {
    fn tick(&mut self, ctx: &mut RLTK::BTerm) {
        ctx.cls();
        ctx.print(1, 1, "Hello Rust World");
    }
}

fn main() -> RLTK::BError {
    let context = RLTK::BTermBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    let gs = State {};
    RLTK::main_loop(context, gs)
}
