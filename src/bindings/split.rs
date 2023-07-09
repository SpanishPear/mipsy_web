use bounce::Atom;
use js_sys::{Object, Reflect};
use wasm_bindgen::{prelude::*, JsCast};
use yew::UseStateHandle;

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
    let split_elements = SplitElements(vec!["#runtime_top", "#runtime_bottom"]);
    let split_options = js_sys::Object::new();
    Reflect::set(
        &split_options,
        &JsValue::from("sizes"),
        &(js_sys::Array::of2(&JsValue::from(50_f64), &JsValue::from(50_f64))),
    )
    .expect("failed to set runtime splits");

    Reflect::set(
        &split_options,
        &JsValue::from("direction"),
        &JsValue::from("vertical"),
    )
    .expect("failed to set runtime splits direction");

    log::debug!("split_elements: {:?}", split_options);

    Split(split_elements.into(), split_options);

    // Initialize split columns
    let split_elements = SplitElements(vec!["#left", "#middle", "#right"]);

    // set the options
    // starter percents of 5, 90, 5
    let options: Object = js_sys::Object::new();
    Reflect::set(
        &options,
        &JsValue::from("sizes"),
        &(js_sys::Array::of3(
            &JsValue::from(5_f64),
            &JsValue::from(50_f64),
            &JsValue::from(45_f64),
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

    let get_sizes_fn = Reflect::get(split_handle, &JsValue::from("getSizes"))
        .unwrap()
        .dyn_into::<js_sys::Function>()
        .unwrap();

    let sizes = get_sizes_fn.call0(split_handle).unwrap();
    // unpack the three values from the array
    let left = js_sys::Reflect::get(&sizes, &JsValue::from(0)).unwrap().as_f64().unwrap();
    let middle = js_sys::Reflect::get(&sizes, &JsValue::from(1)).unwrap().as_f64().unwrap();
    let right = js_sys::Reflect::get(&sizes, &JsValue::from(2)).unwrap().as_f64().unwrap();

    log::info!("{left}, {middle}, {right}");

    // subtract a value of 7% from from the middle pane 
    // and add it to the left pane

    let secondary_panel_size = 10_f64;

    let values = if !*showing {
        js_sys::Array::of3(
            &JsValue::from(left + secondary_panel_size),
            &JsValue::from(middle - secondary_panel_size),
            &JsValue::from(right),
        )
    } else {
        js_sys::Array::of3(
            &JsValue::from(left - secondary_panel_size),
            &JsValue::from(middle + secondary_panel_size),
            &JsValue::from(right),
        )
    };

    // flip the state determining if panel is showing
    showing.set(!(*showing));

    // call the setSizes Function
    set_sizes_fn.call1(split_handle, &values).unwrap();
}
