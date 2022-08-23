use bracket_lib::prelude as RLTK;
use specs::prelude::*;
use specs_derive::*;

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Renderable {
    pub glyph: RLTK::FontCharType,
    pub fg: RLTK::RGB,
    pub bg: RLTK::RGB,
}

#[derive(Component, Debug)]
pub struct Player {}
