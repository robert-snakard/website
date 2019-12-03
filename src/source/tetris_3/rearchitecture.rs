use wasm_bindgen::prelude::*;

mod app;
mod game;
mod pieces;

use crate::app::*;
use crate::game::*;
use crate::pieecs::*;

#[wasm_bindgen(start)]
pub fn run() {
    let ctx = get_context();
    draw_piece(ctx, Tetronimo::T, Rotation::Up, 3, 7);
}

mod app {
    use web_sys::*;
    use wasm_bindgen::JsCast;

    fn get_context() -> CanvasRenderingContext2d {
        // Get the Canvas DOM element
        let window = window().unwrap();
        let document = window.document().unwrap();
        let anonymous_element = document.get_element_by_id("my_canvas").unwrap();
        let canvas = anonymous_element.dyn_into::<HtmlCanvasElement>().unwrap();

        // Get the drawing context
        let ctx = canvas.get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        return ctx;
    }
}

mod game {
    use web_sys::*;
    use wasm_bindgen::JsValue;
    use crate::pieces::PIECES;

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
}

mod pieces {
#[derive(Debug, Copy, Clone)]
    enum Tetronimo {
        L,
        O,
        S,
        I,
        J,
        T,
    }

#[derive(Debug, Copy, Clone)]
    enum Rotation {
        Up,
        Left,
        Down,
        Right,
    }

    pub const PIECES: [[[u8; 16]; 4]; 7] = [
    [// Z
        [1, 1, 0, 0,
        0, 1, 1, 0,
        0, 0, 0, 0,
        0, 0, 0, 0],
        [0, 0, 1, 0,
        0, 1, 1, 0,
        0, 1, 0, 0,
        0, 0, 0, 0],
        [0, 0, 0, 0,
        1, 1, 0, 0,
        0, 1, 1, 0,
        0, 0, 0, 0],
        [0, 1, 0, 0,
        1, 1, 0, 0,
        1, 0, 0, 0,
        0, 0, 0, 0]
    ], [// L
        [0, 0, 1, 0,
        1, 1, 1, 0,
        0, 0, 0, 0,
        0, 0, 0, 0],
        [0, 1, 0, 0,
        0, 1, 0, 0,
        0, 1, 1, 0,
        0, 0, 0, 0],
        [0, 0, 0, 0,
        1, 1, 1, 0,
        1, 0, 0, 0,
        0, 0, 0, 0],
        [1, 1, 0, 0,
        0, 1, 0, 0,
        0, 1, 0, 0,
        0, 0, 0, 0],
    ], [// O
        [0, 1, 1, 0,
        0, 1, 1, 0,
        0, 0, 0, 0,
        0, 0, 0, 0],
        [0, 1, 1, 0,
        0, 1, 1, 0,
        0, 0, 0, 0,
        0, 0, 0, 0],
        [0, 1, 1, 0,
        0, 1, 1, 0,
        0, 0, 0, 0,
        0, 0, 0, 0],
        [0, 1, 1, 0,
        0, 1, 1, 0,
        0, 0, 0, 0,
        0, 0, 0, 0],
    ], [// S
        [0, 1, 1, 0,
        1, 1, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0],
        [0, 1, 0, 0,
        0, 1, 1, 0,
        0, 0, 1, 0,
        0, 0, 0, 0],
        [0, 0, 0, 0,
        0, 1, 1, 0,
        1, 1, 0, 0,
        0, 0, 0, 0],
        [1, 0, 0, 0,
        1, 1, 0, 0,
        0, 1, 0, 0,
        0, 0, 0, 0],
    ], [// I
        [0, 0, 0, 0,
        1, 1, 1, 1,
        0, 0, 0, 0,
        0, 0, 0, 0],
        [0, 0, 1, 0,
        0, 0, 1, 0,
        0, 0, 1, 0,
        0, 0, 1, 0],
        [0, 0, 0, 0,
        0, 0, 0, 0,
        1, 1, 1, 1,
        0, 0, 0, 0],
        [0, 1, 0, 0,
        0, 1, 0, 0,
        0, 1, 0, 0,
        0, 1, 0, 0],
    ], [// J
        [1, 0, 0, 0,
        1, 1, 1, 0,
        0, 0, 0, 0,
        0, 0, 0, 0],
        [0, 1, 1, 0,
        0, 1, 0, 0,
        0, 1, 0, 0,
        0, 0, 0, 0],
        [0, 0, 0, 0,
        1, 1, 1, 0,
        0, 0, 1, 0,
        0, 0, 0, 0],
        [0, 1, 0, 0,
        0, 1, 0, 0,
        1, 1, 0, 0,
        0, 0, 0, 0],
    ], [// T
        [0, 1, 0, 0,
        1, 1, 1, 0,
        0, 0, 0, 0,
        0, 0, 0, 0],
        [0, 1, 0, 0,
        0, 1, 1, 0,
        0, 1, 0, 0,
        0, 0, 0, 0],
        [0, 0, 0, 0,
        1, 1, 1, 0,
        0, 1, 0, 0,
        0, 0, 0, 0],
        [0, 1, 0, 0,
        1, 1, 0, 0,
        0, 1, 0, 0,
        0, 0, 0, 0],
    ]];
}
