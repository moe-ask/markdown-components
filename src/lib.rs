use wasm_bindgen::prelude::*;
use tera::{Context, Tera};

#[cfg(feature = "wee-alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn render(ctx: JsValue, content: String) -> String {
    console_error_panic_hook::set_once();
    let context = Context::from_serialize(ctx.into_serde::<serde_json::Value>().unwrap()).unwrap();

    let mut tera = Tera::default();
    tera.autoescape_on(Vec::new());

    tera.render_str(&content, &context).unwrap()
}