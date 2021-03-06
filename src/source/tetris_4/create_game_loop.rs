use web_sys::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen::JsCast;
use gloo_timers::callback::Interval;

mod app;
mod game;
mod pieces;

use crate::app::*;
use crate::game::*;
use crate::pieces::*;

#[wasm_bindgen(start)]
pub fn run() {
    // Game Loop
    Interval::new(1000/60, move || { game_loop(); }).forget();
}

fn game_loop() {
    let ctx = get_context();
    draw_piece(ctx, Tetronimo::T, Rotation::Up, 3, 7);
}
