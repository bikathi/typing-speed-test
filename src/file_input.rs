use gloo::file::{File, callbacks::FileReader};
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};
use yew::prelude::*;

pub(crate) enum ComponentMsg {
    FileSelected(Event),
    FileLoaded(String, String), // (filename, content)
}

pub(crate) struct FileInput {
    reader: Option<FileReader>,
}

impl Component for FileInput {
    type Message = ComponentMsg;
    type Properties = crate::app::FileInputProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { reader: None }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ComponentMsg::FileSelected(e) => {
                let input: HtmlInputElement = e.target().unwrap().dyn_into().unwrap();

                if let Some(file) = input.files().and_then(|files| files.get(0)) {
                    let file = File::from(file);
                    let filename = file.name();

                    let link = ctx.link().clone();
                    let reader = gloo::file::callbacks::read_as_text(&file, move |res| {
                        if let Ok(content) = res {
                            link.send_message(ComponentMsg::FileLoaded(filename.clone(), content));
                        }
                    });

                    self.reader = Some(reader);
                }
                false
            }
            ComponentMsg::FileLoaded(_filename, content) => {
                // log::info!("File: {}\nContent:\n{}", filename, content);
                ctx.props().on_file_contents_load.emit(content);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onchange = ctx.link().callback(ComponentMsg::FileSelected);

        html! {
            <label
                for={"dropzone-file"}
                class="flex flex-col items-center justify-center h-20 w-64 border-2 border-accent hover:border-accent/80 hover:bg-accent-content/80 border-dashed rounded-lg cursor-pointer bg-accent-content">
                <div class="flex flex-col items-center justify-center pt-5 pb-6">
                    <p class="mb-2 text-sm text-gray-500"><span class="font-semibold">{"Click to upload"}</span></p>
                    <p class="text-xs text-gray-500">{"Only .txt files supported"}</p>
                </div>
                <input id="dropzone-file" type="file" class="hidden" {onchange} accept=".txt" />
            </label>
        }
    }
}
