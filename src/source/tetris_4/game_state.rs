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

struct GameState {
    frame_count: usize,
    ycoord: usize,
}

#[wasm_bindgen(start)]
pub fn run() {
    // Game Loop
    let mut state = GameState { frame_count: 0,
                                ycoord: 0 };
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

mod game {
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

    pub fn clear_board(ctx: &CanvasRenderingContext2d) {
        const BOARD_WIDTH: usize = 100;
        const BOARD_HEIGHT: usize = 200;

        ctx.set_fill_style(&JsValue::from("black"));
        ctx.fill_rect(0.0, 0.0, BOARD_WIDTH as f64,
                      BOARD_HEIGHT as f64);
    }
}
