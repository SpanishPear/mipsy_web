use bounce::use_slice;
use gloo::file::File;
use stylist::yew::styled_component;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::editor::files::{FileList, FileListAction};

#[styled_component(OpenFilesButton)]
pub fn render() -> Html {
    let files = use_slice::<FileList>();
    let onchange = {
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let files = files.clone();
            // for every file, read it and
            // add it to the FileList
            let mut result = Vec::new();

            if let Some(files) = input.files() {
                let files = js_sys::try_iter(&files)
                    .unwrap()
                    .unwrap()
                    .map(|v| web_sys::File::from(v.unwrap()))
                    .map(File::from);
                result.extend(files);
            }

            for file in result.into_iter() {
                let name = file.name();
                let contents = gloo::file::futures::read_as_bytes(&file);
                let files = files.clone();
                spawn_local(async move {
                    let contents = contents.await;

                    // TODO(error handling): handle invalid utf8
                    let contents = contents.expect("File contains invalid utf8");

                    let contents = String::from_utf8(contents).unwrap();

                    files.dispatch(FileListAction::Append(name, contents));

                    files.dispatch(FileListAction::Log);
                });
            }
        })
    };

    html! {
        <>
            <label for="load_files" tabindex=0>
                <OpenFileIcon />
            </label>
            <input class={css!("display: none;")} type="file" id="load_files" multiple=true {onchange} />
        </>
    }
}

#[styled_component(OpenFileIcon)]
fn render_icon() -> Html {
    html! {
       <svg version="1.1" xmlns="http://www.w3.org/2000/svg" x="0px" y="0px"
        width="592.813px" height="592.814px" viewBox="0 0 592.813 592.814" style="enable-background:new 0 0 592.813 592.814;"
        class={css!(r#"
            width: 1rem;
            height: 1rem;
            cursor: pointer;
            transition: fill 0.2s ease-in-out;
            &:hover {
                fill: #000;
        }"#)}>
           <g>
             <path d="M589.173,356.232l-104.756,198.26c-5.125,9.858-19.653,20.285-30.872,20.285l-420.096,0.077
               c-8.875,0-17.384-3.518-23.655-9.794C3.523,558.783,0,550.283,0,541.405l0.068-326.209c0-18.448,14.955-33.417,33.405-33.435
               l30.715-0.029v28.496H43.639c-4.022,0-7.885,1.596-10.731,4.442c-2.843,2.846-4.442,6.706-4.442,10.731l0.03,305.796
               c0,8.388,6.797,15.173,15.176,15.173h21.045l99.28-200.836c5.609-11.219,16.208-20.286,27.411-20.286h243.14l0.083-80.823
               c15.876,1.472,28.406,14.641,28.406,30.893v49.931H574.55C587.719,325.384,598.808,338.406,589.173,356.232z M83.558,445.272
               c-0.907-99.969,0-399.884,0-399.884c0-15.132,12.306-27.429,27.423-27.429h219.614c3.518,0,6.874,1.472,9.251,4.061l71,77.141
               c2.128,2.323,3.321,5.364,3.321,8.515v199.839h-23.034V124.932c0-3.159-2.565-5.725-5.728-5.725h-54.343
               c-6.36,0-11.532-5.163-11.532-11.511V46.721c0-3.16-2.565-5.725-5.728-5.725H110.995c-2.423,0-4.395,1.971-4.395,4.392v374.739
               l-17.626,35.66C88.975,455.781,83.649,455.391,83.558,445.272z M342.588,96.182H376.8l-34.212-37.188V96.182z M355.065,142.667
               H142.813c-7.82,0-14.168,6.354-14.168,14.174c0,7.814,6.354,14.171,14.168,14.171h212.258c7.82,0,14.187-6.362,14.187-14.171
               C369.245,149.027,362.88,142.667,355.065,142.667z M369.245,239.376c0-7.814-6.359-14.162-14.18-14.162H142.813
               c-7.82,0-14.168,6.36-14.168,14.162c0,7.814,6.354,14.162,14.168,14.162h212.258C362.88,253.539,369.245,247.19,369.245,239.376z
                M128.636,322.47c0,7.813,6.357,14.162,14.171,14.162h5.089c8.958-24.967,31.164-28.324,31.164-28.324h-36.253
               C135.005,308.308,128.636,314.656,128.636,322.47z"/>
           </g>
       </svg>

    }
}
