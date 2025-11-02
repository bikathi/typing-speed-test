use crate::settings::AppSettings;
use yew::prelude::*;

mod file_input;
mod settings;
mod util;

#[function_component]
fn App() -> Html {
    let app_settings = use_state(move || AppSettings::default());
    let source_text = use_state(move || String::from("this is a sample statement"));
    let user_input = use_state(move || Vec::new());

    let increase_font_size = {
        if app_settings.font_size < settings::MAX_FONT_SIZE {
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
        if app_settings.font_size > settings::MIN_FONT_SIZE {
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

    let on_key_board_input = {
        let current_user_input = user_input.clone();
        Callback::from(move |e: KeyboardEvent| {
            let mut new_input = (*current_user_input).clone();

            if !e.key().eq(&("Backspace".to_string())) {
                new_input.push(e.key().chars().nth(0).unwrap());
            } else {
                if new_input.len() > 0 {
                    new_input.remove(new_input.len() - 1);
                }
            }

            current_user_input.set(new_input);
        })
    };

    let reset_state = {
        let source_text = source_text.clone();
        let user_input = user_input.clone();

        Callback::from(move |_e| {
            source_text.set(String::from("this is a sample statement"));
            user_input.set(Vec::new());
        })
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
                        <file_input::FileInput />
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
                    <p tabindex=0 class={classes!(format!("z-10 w-full h-96 mt-5 input-areas {}", {
                        match app_settings.font_size {
                            10 => "text-xl",
                            20 => "text-2xl",
                            30 => "text-3xl",
                            40 => "text-4xl",
                            50 => "text-5xl",
                            _ => ""
                        }
                    }))} onkeydown={on_key_board_input}>
                        {
                            source_text.char_indices().map(|(index, c)| {
                                let class={
                                    if !user_input.is_empty() {
                                        match user_input.get(index) {
                                            Some(user_input) => {
                                                if user_input.to_owned() == c {
                                                    "text-slate-100"
                                                } else {
                                                    "underline underline-offset-2 text-red-500 decoration-red-500"
                                                }
                                            },
                                            None => "text-base-content/50",
                                        }
                                    } else {
                                        "text-base-content/50"
                                    }
                                };

                                let animated_cursor_pre = {
                                    if (index == 0 && user_input.len() == 0) || (index == user_input.len()) {
                                        "current"
                                    } else {
                                        ""
                                    }
                                };
                                html! {
                                    <span
                                        key={ index }
                                        class={format!("{} {} relative", class, animated_cursor_pre)}
                                    >
                                        { format!("{c}") }
                                    </span>
                                }
                            }).collect::<Html>()
                        }
                    </p>

                    // reset button
                    <div class={classes!(String::from("mt-5 text-center h-fit"))}>
                        <button class={classes!(String::from("settings-button rounded-full"))} title="Reset" onclick={reset_state}>
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
    // init logger
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
}
