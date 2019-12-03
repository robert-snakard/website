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

mod game {
    pub struct GameState {
        frame_count: usize,
        ycoord: usize,
        xcoord: usize,
    }

    impl GameState {
        pub fn new() -> GameState {
            GameState {
                frame_count: 0,
                ycoord: 0,
                xcoord: 3,
            }
        }
    }

    pub fn game_loop(mut state: RefMut<GameState>) {
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

    pub fn move_piece(event: &KeyboardEvent, mut state: RefMut<GameState>) {
        match event.key_code() {
            37 => if state.xcoord > 0 { state.xcoord -= 1; }
            39 => if state.xcoord < 7 { state.xcoord += 1; }
            _ => {},
        }
    }

    fn draw_piece(ctx: CanvasRenderingContext2d,
            shape: Tetronimo, rot: Rotation,
            xcoord: usize, ycoord: usize) {

        // Get Piece
        let piece = PIECES[shape as usize]
            [rot as usize];
        ctx.set_fill_style(&JsValue::from(format!(
                        "hsl({}, 50%, 50%)", 45*(shape as usize)
                        )));

        // Set aside piece dimensions
        const PIECE_WIDTH: usize = 4;
        const SQUARE_SIZE: f64 = 10.0;

        // Loop over the piece array and draw each square
        for i in 0..piece.len() {
            let spot_value = piece[i];

            if spot_value == 1 {
                let xcoord = i % PIECE_WIDTH;
                let ycoord = i / PIECE_WIDTH;
                ctx.fill_rect(xcoord as f64 * SQUARE_SIZE,
                        ycoord as f64 * SQUARE_SIZE,
                        SQUARE_SIZE, SQUARE_SIZE);

            }
        }
    }

    fn clear_board(ctx: &CanvasRenderingContext2d) {
        const BOARD_WIDTH: usize = 100;
        const BOARD_HEIGHT: usize = 200;

        ctx.set_fill_style(&JsValue::from("black"));
        ctx.fill_rect(0.0, 0.0, BOARD_WIDTH as f64,
                BOARD_HEIGHT as f64);
    }
}
