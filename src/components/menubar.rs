use bounce::{use_atom, use_slice};
use gloo::file::File;
use stylist::yew::styled_component;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::app::{FileInfo, FileList, FileListAction};

#[styled_component(MenuBar)]
pub fn menubar() -> Html {
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
        <input
            id="file-upload"
            type="file"
            accept=".s"
            // This only allows folders to be selected,
            // not files.
            // webkitdirectory={Some("")}
            multiple={true}
            {onchange}
        />
    }
}
