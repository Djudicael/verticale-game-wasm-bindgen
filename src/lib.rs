use canvas::run_canvas;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
mod canvas;
mod models;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    spawn_local(async move {
        run_canvas(&document).await.unwrap_throw();
    });
    Ok(())
}
