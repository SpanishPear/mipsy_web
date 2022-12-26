pub fn setup() {
    use mipsy_web::components::app::AppRoot;

    // basically we are injecting javascript into the page

    let head = gloo_utils::document().head().expect("no head");
    let script = gloo_utils::document().create_element("script").unwrap();
    script
        .append_child(
            &gloo_utils::document().create_text_node(include_str!("../../assets/split.js")),
        )
        .unwrap();

    head.append_child(&script).unwrap();

    yew::Renderer::<AppRoot>::new().render();
}
