//! [Web-sys docs](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.CanvasRenderingContext2d.html)
//! [Web-sys example](https://rustwasm.github.io/wasm-bindgen/examples/2d-canvas.html)
//! [MDN](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawWindow)

use seed::{prelude::*, *};
mod canvas;
mod canvas_demo;
// ------ ------
//     Start
// ------ ------

//#[wasm_bindgen(start)]
pub fn start() {
    canvas::start();
    // App::start("app", init, update, view);
}
