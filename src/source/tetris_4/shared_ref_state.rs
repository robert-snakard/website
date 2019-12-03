use web_sys::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen::JsCast;
use gloo_timers::callback::Interval;
use gloo_events::EventListener;

use std::rc::Rc;
use std::cell::{RefCell, RefMut};

mod app;
mod game;
mod pieces;

use crate::app::*;
use crate::game::*;
use crate::pieces::*;

struct GameState {
    frame_count: usize,
    ycoord: usize,
    xcoord: usize,
}

#[wasm_bindgen(start)]
pub fn run() {
    let state1 = Rc::new(RefCell::new(GameState { 
                                          frame_count: 0,
                                          ycoord: 0,
                                          xcoord: 3,
                                        }));
    let state2 = state1.clone();
                                
    EventListener::new(&window().unwrap(), "keydown", move |event| {
        move_piece(event.dyn_ref::<KeyboardEvent>().unwrap(), state2.borrow_mut());
    }).forget();

    // Game Loop
    Interval::new(1000/60, move || { 
        game_loop(state1.borrow_mut()); 
    }).forget();
}

fn game_loop(mut state: RefMut<GameState>) {
    let ctx = get_context();

    state.frame_count += 1;
    if state.frame_count == 48 {
        state.frame_count = 0;
        state.ycoord += 1;
    }

    clear_board(&ctx);
    draw_piece(&ctx, Tetronimo::T, 
               Rotation::Up, 
               state.xcoord, state.ycoord);
}

fn move_piece(event: &KeyboardEvent, mut state: RefMut<GameState>) {
    match event.key_code() {
        37 => if state.xcoord > 0 { state.xcoord -= 1; }
        39 => if state.xcoord < 7 { state.xcoord += 1; }
        _ => {},
    }
}
