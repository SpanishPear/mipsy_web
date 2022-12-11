#![allow(clippy::let_unit_value)]
pub mod agent;
pub mod components;
pub mod config;
pub mod editor;
pub mod indexdb_fs;
pub mod state;
use bounce::Atom;
use js_sys::{Object, Reflect};
use wasm_bindgen::{prelude::*, JsCast};
use yew::UseStateHandle;

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

#[derive(Atom, Default, PartialEq)]
pub struct SplitContainer {
    pub handle: JsValue,
}

/// setup the initial splits and return the handle
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
            &JsValue::from(3_f64),
            &JsValue::from(47_f64),
            &JsValue::from(50_f64),
        )),
    )
    .expect("Creating options via reflection failed");

    // set a minsize of 0

    _ = Reflect::set(&options, &JsValue::from("minSize"), &JsValue::from(50_f64));

    Split(split_elements.into(), options)
}

/// Checks if the secondary panel is showing,
/// and if so, it closes the panel. If it is not showing,
/// it opens the panel.
///
/// Done via a callback to the SplitJs handle
pub fn toggle_secondary_pane(split_handle: &JsValue, showing: UseStateHandle<bool>) {
    let set_sizes_fn = Reflect::get(split_handle, &JsValue::from("setSizes"))
        .unwrap()
        .dyn_into::<js_sys::Function>()
        .unwrap();

    let values = if !*showing {
        js_sys::Array::of3(&JsValue::from(10), &JsValue::from(40), &JsValue::from(50))
    } else {
        js_sys::Array::of3(&JsValue::from(3), &JsValue::from(47), &JsValue::from(50))
    };

    // flip the state determining if panel is showing
    showing.set(!(*showing));

    // call the setSizes Function
    let _resize = set_sizes_fn.call1(split_handle, &values).unwrap();
}
