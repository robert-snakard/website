use web_sys::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen::JsCast;

#[wasm_bindgen(start)]
pub fn hello_world() {
    let window = window().unwrap();
    let document = window.document().unwrap();
    let anonymous_element = document.get_element_by_id("my_canvas").unwrap();
    let canvas = anonymous_element.dyn_into::<HtmlCanvasElement>().unwrap();
    
    let ctx = canvas.get_context("2d")
                    .unwrap()
                    .unwrap()
                    .dyn_into::<CanvasRenderingContext2d>()
                    .unwrap();

    ctx.set_fill_style(&JsValue::from("red"));
    ctx.fill_rect(10.0, 10.0, 10.0, 10.0);
}
