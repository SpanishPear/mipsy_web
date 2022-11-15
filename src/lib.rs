pub mod agent;
pub mod components;
pub mod editor;
pub mod indexdb_fs;
use js_sys::{Object, Reflect};
use wasm_bindgen::prelude::*;

//TODO: create a SplitJs rust binding
#[wasm_bindgen]
extern "C" {
    /// Documentation at https://github.com/nathancahill/split/tree/master/packages/splitjs
    pub fn Split(elements: js_sys::Array, options: js_sys::Object) -> JsValue;
}

pub struct SplitElements(pub Vec<&'static str>);
impl From<SplitElements> for js_sys::Array {
    fn from(v: SplitElements) -> Self {
        v.0.into_iter()
            .map(|s| s.to_string())
            .map(JsValue::from)
            .collect()
    }
}

pub fn setup_splits() -> JsValue {
    // Initialize split panes
    let split_elements = SplitElements(vec!["#left", "#middle", "#right"]);

    // set the options
    // starter percents of 5, 90, 5
    let options: Object = js_sys::Object::new();
    Reflect::set(
        &options,
        &JsValue::from("sizes"),
        &(js_sys::Array::of3(
            &JsValue::from(5_f64),
            &JsValue::from(90_f64),
            &JsValue::from(5_f64),
        )),
    )
    .expect("Creating options via reflection failed");

    Split(split_elements.into(), options)
}
