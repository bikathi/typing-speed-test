use crate::settings::AppSettings;
use yew::prelude::*;

mod settings;

#[function_component]
fn App() -> Html {
    let app_settings = use_state(move || AppSettings::default());

    let increase_font_size = {
        if app_settings.font_size < 50 {
            let app_settings = app_settings.clone();
            Callback::from(move |_e| {
                let mut new_app_settings = (*app_settings).clone(); // get an instance we can modify
                new_app_settings.inc_font_size(); // modify whatever we want to modify
                app_settings.set(new_app_settings); // update state
            })
        } else {
            Callback::default()
        }
    };

    let decrease_font_size = {
        if app_settings.font_size > 10 {
            let app_settings = app_settings.clone();
            Callback::from(move |_e| {
                let mut new_app_settings = (*app_settings).clone(); // get an instance we can modify
                new_app_settings.dec_font_size(); // modify whatever we want to modify
                app_settings.set(new_app_settings); // update state
            })
        } else {
            Callback::default()
        }
    };

    html! {
        <main class={classes!(String::from("flex justify-center h-screen bg-base-100"))}>
            <div class={classes!(String::from("flex flex-col justify-between h-full w-full md:w-1/2"))}>
                <div>/**/</div>


                <div class={classes!(String::from("h-fit"))}>
                    // allow users to upload their own source file
                    <form class={classes!(String::from("flex items-center justify-between"))}>
                        <span class={classes!(String::from("text-4xl text-base-content"))}>
                            {"05:00"}
                        </span>
                        <button class={classes!(String::from("py-2 px-3 h-14 rounded-md font-semibold text-sm outline-none bg-secondary/80 hover:bg-secondary/100 transition-colors duration-200 ease-in-out text-base-content"))}>
                            {"Custom Text File"}
                        </button>
                    </form>

                    // settings
                    <div class={classes!(String::from("mt-2 py-4 flex justify-between items-center space-x-2"))}>
                        // font size
                        <div class={classes!(String::from("w-fit"))}>
                            <h1 class={classes!(String::from("text-sm mb-1 text-base-content font-semibold"))}>{"Font Size"}</h1>
                            <div class={classes!(String::from("divide-x-1 rounded-lg overflow-clip"))}>
                                <p class={classes!(String::from("h-14 w-20 inline-flex items-center justify-center text-xl bg-red-500"))}>{ app_settings.font_size }</p>
                                <button class={classes!(String::from("settings-button"))} onclick={increase_font_size}>{"+"}</button>
                                <button class={classes!(String::from("settings-button"))} onclick={decrease_font_size}>{"-"}</button>
                            </div>
                        </div>

                        // theme
                        <div class={classes!(String::from("w-fit"))}>
                            <h1 class={classes!(String::from("text-sm mb-1 text-base-content font-semibold"))}>{"Theme"}</h1>
                            <div class={classes!(String::from("divide-x-1 rounded-lg overflow-clip"))}>
                                <button class={classes!(String::from("settings-button"))}>
                                    <span class="icon-[mdi-light--lightbulb] text-3xl"></span>
                                </button>
                                <button class={classes!(String::from("settings-button"))}>
                                    <span class="icon-[mdi-light--lightbulb-on] text-3xl"></span>
                                </button>
                            </div>
                        </div>

                        // time
                        <div class={classes!(String::from("w-fit"))}>
                            <h1 class={classes!(String::from("text-sm mb-1 text-base-content font-semibold"))}>{"Time"}</h1>
                            <div class={classes!(String::from("divide-x-1 rounded-lg overflow-clip"))}>
                                <button class={classes!(String::from("settings-button"))}>{"+"}</button>
                                <button class={classes!(String::from("settings-button"))}>{"-"}</button>
                            </div>
                        </div>

                        // timer
                        <div class={classes!(String::from("w-fit"))}>
                            <h1 class={classes!(String::from("text-sm mb-1 text-base-content font-semibold"))}>{"Timer"}</h1>
                            <button class={classes!(String::from("settings-button rounded-full"))}>
                                <span class="icon-[mdi-light--play] text-3xl"></span>
                                // <span class="icon-[mdi-light--pause]"></span>
                            </button>
                        </div>
                    </div>

                    // typing input
                    <form class={classes!(String::from("w-full h-96 mt-5 relative"))}>
                        <p class={classes!(format!("text-base-content/50 z-10 input-areas {}", {
                            match app_settings.font_size {
                                10 => "text-xl",
                                20 => "text-2xl",
                                30 => "text-3xl",
                                40 => "text-4xl",
                                50 => "text-5xl",
                                _ => ""
                            }
                        }))}>
                            { "this is a sample statement" }
                        </p>
                        <textarea class={classes!(format!("text-base-content {} z-20 bg-transparent input-areas", {
                            match app_settings.font_size {
                                10 => "text-xl",
                                20 => "text-2xl",
                                30 => "text-3xl",
                                40 => "text-4xl",
                                50 => "text-5xl",
                                _ => ""
                            }
                        }))} />
                    </form>

                    // reset button
                    <div class={classes!(String::from("mt-5 text-center h-fit"))}>
                        <button class={classes!(String::from("settings-button rounded-full"))} title="Reset">
                            <span class="icon-[mdi-light--refresh] text-3xl"></span>
                        </button>
                    </div>
                </div>

                <footer class={classes!(String::from("h-10 text-center"))}>
                    <p class={classes!(String::from("text-sm text-base-content"))}>{"Powered By YewRS"}</p>
                </footer>
            </div>


        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
