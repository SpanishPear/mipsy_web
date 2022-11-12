use crate::editor::get_options;
use monaco::api::TextModel;
use monaco::sys::Uri;
use monaco::yew::CodeEditor;
use stylist::yew::styled_component;
use stylist::StyleSource;
use yew::prelude::*;

#[derive(Properties, PartialEq, Eq)]
pub struct EditorProps {
    pub styles: String,
}

#[styled_component(Editor)]
pub fn editor(EditorProps { styles }: &EditorProps) -> Html {
    let styles: StyleSource = styles.as_str().into();
    let models: UseStateHandle<Vec<String>> = use_state(Vec::new);
    let current_model: UseStateHandle<String> = use_state(|| {
        let uri = TextModel::create("test", "mips".into(), None)
            .unwrap()
            .uri()
            .to_string(false);

        // add the model to the models hash_map
        let mut models_vec = (*models).clone();
        models_vec.push(uri.clone());
        models.set(models_vec);

        uri
    });
    log::info!("models: {:?}", models);
    let button_onclick2: Callback<MouseEvent> = {
        // get the second uri in the vec, if it exists
        // if it doesn't exist, create a new model 
        let models = models.clone();
        let current_model = current_model.clone();
        Callback::from(move |_| {
       
            log::info!("button clicked");
            let mut models_vec = (*models).clone();
            let uri = if let Some(uri) = models_vec.get(1) {
                uri.clone()
            } else {
                let uri = TextModel::create("TAB 2", "mips".into(), None)
                    .unwrap()
                    .uri()
                    .to_string(false);
                models_vec.push(uri.clone());
                models.set(models_vec);
                uri
            };
            
            current_model.set(uri);
        })
    };

    let button_onclick: Callback<MouseEvent> = {
        let models = models.clone();
        let current_model = current_model.clone();
        Callback::from(move |_| {
       
            log::info!("button clicked");
            let mut models_vec = (*models).clone();
            let uri = if let Some(uri) = models_vec.get(0) {
                uri.clone()
            } else {
                let uri = TextModel::create("TAB 1", "mips".into(), None)
                    .unwrap()
                    .uri()
                    .to_string(false);
                models_vec.push(uri.clone());
                models.set(models_vec);
                uri
            };
            
            current_model.set(uri);
        })
    };

    let current_uri: Uri = Uri::parse(&(*current_model), false);
    let model = TextModel::get(&current_uri).unwrap();
    html! {
        <>
            <button onclick={button_onclick}>{"Tab 1"}</button>
            <button onclick={button_onclick2}>{"Tab 2"}</button>
            <CodeEditor
                classes={styles}
                options={get_options()}
                model={model}
            />
        </>
    }
}
