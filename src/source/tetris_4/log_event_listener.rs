use web_sys::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen::JsCast;
use gloo_timers::callback::Interval;
use gloo_events::EventListener;

mod app;
mod game;
mod pieces;

use crate::app::*;
use crate::game::*;
use crate::pieces::*;

struct GameState {
    frame_count: usize,
    ycoord: usize,
}

#[wasm_bindgen(start)]
pub fn run() {
    let mut state = GameState { frame_count: 0,
                                ycoord: 0 };
                                
    EventListener::new(&window().unwrap(), "keydown", move |event| {
        console::log_1(event);
    }).forget();

    // Game Loop
    Interval::new(1000/60, move || { 
        game_loop(&mut state); 
    }).forget();
}

fn game_loop(state: &mut GameState) {
    let ctx = get_context();

    state.frame_count += 1;
    if state.frame_count == 48 {
        state.frame_count = 0;
        state.ycoord += 1;
    }

    clear_board(&ctx);
    draw_piece(&ctx, Tetronimo::T, 
               Rotation::Up, 
               3, state.ycoord);
}
