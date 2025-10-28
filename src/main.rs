use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0i32);
    let on_increase_click = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    let on_decrease_click = {
        let counter = counter.clone();
        move |_| {
            let value = *counter - 1;
            counter.set(value);
        }
    };

    html! {
        <div>
            <button 
                onclick={on_increase_click} 
                class={classes!(String::from("bg-green-500 size-20 m-2 rounded-md shadow-sm hover:scale-105 transition-all duration-150 ease-in-out"))}>
                { "+1" }
            </button>
            <p>{ *counter }</p>
            <button 
                onclick={on_decrease_click} 
                class={classes!(String::from("bg-red-500 size-20 m-2 rounded-md shadow-sm hover:scale-105 transition-all duration-150 ease-in-out"))}>
                { "-1" }
            </button>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
