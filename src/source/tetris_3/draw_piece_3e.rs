use web_sys::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen::JsCast;

#[wasm_bindgen(start)]
pub fn run() {
    draw_piece();
}

fn draw_piece() {
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

    // Draw a square to the screen
    ctx.set_fill_style(&JsValue::from("red"));
    ctx.fill_rect(10.0, 10.0, 10.0, 10.0);

    // TODO: Draw Piece
    let piece = [
        0, 1, 0, 0,
        1, 1, 1, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
    ];

    // Set aside piece dimensions
    const PIECE_WIDTH: usize = 4;
    const SQUARE_SIZE: f64 = 10.0;

    // Loop over the piece array and draw each square
    for i in 0..piece.len() {
        let spot_value = piece[i];
        
        if spot_value == 1 {
            let xcoord = i % 4;
            let ycoord = i / 4;
            ctx.fill_rect(xcoord as f64 * 10.0,
                          ycoord as f64 * 10.0,
                          10.0, 10.0);
        }
    }
}

fn move_piece() {

}

fn create_piece() {

}

fn detect_collisions() {

}
