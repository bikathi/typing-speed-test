use yew::prelude::*;

pub(crate) struct FileInput {
    file: Option<web_sys::File>,
}

pub enum ComponentMsg {
    SelectedFile(Option<web_sys::File>),
}

impl Component for FileInput {
    type Message = ComponentMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { file: None }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <label
                    class={"mb-2 block text-sm font-medium text-gray-900 dark:text-white"}
                    for={"small_size"}>
                        {"Small file input"}
                </label>
                <input
                class="mb-5 block w-full cursor-pointer rounded-lg border border-gray-300 bg-gray-50 text-xs text-gray-900 focus:outline-none dark:border-gray-600 dark:bg-gray-700 dark:text-gray-400 dark:placeholder-gray-400"
                    id="small_size"
                    type="file"
                   />
            </>
        }
    }
}
