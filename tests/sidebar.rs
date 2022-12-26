mod common;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::HtmlElement;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn file_explorer_opens() {
    common::setup();

    let sidebar_buton: HtmlElement = gloo_utils::document()
        .get_element_by_id("file-explorer-sidebar-button")
        .expect("file explorer sidebar button should exist")
        .dyn_into::<HtmlElement>()
        .unwrap();

    sidebar_buton.click();

    // click the sidebar button
}
