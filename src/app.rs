use crate::utils::AppUtils;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FileInputProps {
    pub on_file_contents_load: Callback<String>,
}

#[derive(Debug, Clone)]
pub(crate) enum ComponentMsg {
    FontSizeIncreased,
    FontSizeDecreased,
    StateReset,
    IncomingUserInput(String),
    UpdateSourceText(String),
    // for the timer
    TimerTick,
    TimerTogglePause,
    TimerAdjustTime(i32),
}

#[derive(Debug, Clone)]
pub(crate) struct App {
    source_text: String,
    user_input: Vec<char>,
    app_settings: AppUtils,
}

impl Component for App {
    type Message = ComponentMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            source_text: String::from(crate::utils::DEFAULT_TEXT),
            user_input: Vec::new(),
            app_settings: AppUtils::default(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_file_contents_load: Callback<String> = ctx
            .link()
            .callback(|file_contents: String| ComponentMsg::UpdateSourceText(file_contents));

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

                            <crate::file_input::FileInput { on_file_contents_load } />
                        </form>

                        // settings
                        <div class={classes!(String::from("mt-2 py-4 flex justify-between items-center space-x-2"))}>
                            // font size
                            <div class={classes!(String::from("w-fit"))}>
                                <h1 class={classes!(String::from("text-sm mb-1 text-base-content font-semibold"))}>{"Font Size"}</h1>
                                <div class={classes!(String::from("divide-x-1 rounded-lg overflow-clip"))}>
                                    <p class={classes!(String::from("h-14 w-20 inline-flex items-center justify-center text-xl border-secondary/80 border-dashed border-r-0 bg-accent-content text-base-content"))}>{ self.app_settings.font_size }</p>
                                    <button class={classes!(String::from("settings-button"))} onclick={ctx.link().callback(|_| ComponentMsg::FontSizeIncreased)}>{"+"}</button>
                                    <button class={classes!(String::from("settings-button"))} onclick={ctx.link().callback(|_| ComponentMsg::FontSizeDecreased)}>{"-"}</button>
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
                        <p id="typing-container" tabindex="0" class={classes!(format!("z-10 w-full h-96 mt-5 input-areas {}", {
                            match self.app_settings.font_size {
                                10 => "text-xl",
                                20 => "text-2xl",
                                30 => "text-3xl",
                                40 => "text-4xl",
                                50 => "text-5xl",
                                _ => ""
                            }
                        }))} onkeydown={ctx.link().callback(|e: KeyboardEvent| ComponentMsg::IncomingUserInput(e.key()))}>
                            {
                                self.source_text.char_indices().map(|(index, c)| {
                                    let class={
                                        if !self.user_input.is_empty() {
                                            match self.user_input.get(index) {
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
                                        if (index == 0 && self.user_input.len() == 0) || (index == self.user_input.len()) {
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
                            <button class={classes!(String::from("settings-button rounded-full"))} title="Reset" onclick={ctx.link().callback(|_| ComponentMsg::StateReset)}>
                                <span class="icon-[mdi-light--refresh] text-3xl"></span>
                            </button>
                        </div>
                    </div>

                    <footer class={classes!(String::from("h-10 justify-center flex items-center space-x-2"))}>
                        <p class={classes!(String::from("text-sm text-base-content"))}>
                            {"Powered by yew.rs"}
                        </p>
                        <a href="https://github.com/bikathi/typing-speed-test" target="_blank" class="hover:scale-105">
                            <svg
                                stroke="#ffffff"
                                fill="#ffffff"
                                height="24"
                                width="24"
                                xmlns="http://www.w3.org/2000/svg"
                                id="mdi-github"
                                viewBox="0 0 24 24">
                                    <path d="M12,2A10,10 0 0,0 2,12C2,16.42 4.87,20.17 8.84,21.5C9.34,21.58 9.5,21.27 9.5,21C9.5,20.77 9.5,20.14 9.5,19.31C6.73,19.91 6.14,17.97 6.14,17.97C5.68,16.81 5.03,16.5 5.03,16.5C4.12,15.88 5.1,15.9 5.1,15.9C6.1,15.97 6.63,16.93 6.63,16.93C7.5,18.45 8.97,18 9.54,17.76C9.63,17.11 9.89,16.67 10.17,16.42C7.95,16.17 5.62,15.31 5.62,11.5C5.62,10.39 6,9.5 6.65,8.79C6.55,8.54 6.2,7.5 6.75,6.15C6.75,6.15 7.59,5.88 9.5,7.17C10.29,6.95 11.15,6.84 12,6.84C12.85,6.84 13.71,6.95 14.5,7.17C16.41,5.88 17.25,6.15 17.25,6.15C17.8,7.5 17.45,8.54 17.35,8.79C18,9.5 18.38,10.39 18.38,11.5C18.38,15.32 16.04,16.16 13.81,16.41C14.17,16.72 14.5,17.33 14.5,18.26C14.5,19.6 14.5,20.68 14.5,21C14.5,21.27 14.66,21.59 15.17,21.5C19.14,20.16 22,16.42 22,12A10,10 0 0,0 12,2Z" />
                            </svg>
                        </a>
                    </footer>
                </div>


            </main>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ComponentMsg::FontSizeDecreased => {
                if self.app_settings.font_size > crate::utils::MIN_FONT_SIZE {
                    self.app_settings.dec_font_size();
                    return true;
                }

                false
            }
            ComponentMsg::FontSizeIncreased => {
                if self.app_settings.font_size < crate::utils::MAX_FONT_SIZE {
                    self.app_settings.inc_font_size();
                    return true;
                }

                false
            }
            ComponentMsg::StateReset => {
                self.source_text = String::from(crate::utils::DEFAULT_TEXT);
                self.user_input = Vec::new();
                self.app_settings.font_size = 30_i32;
                true
            }
            ComponentMsg::IncomingUserInput(key) => {
                if key.eq(&("Shift".to_string())) {
                    return false;
                } else if key.eq(&("Backspace".to_string())) {
                    if self.user_input.len() > 0 {
                        self.user_input.remove(self.user_input.len() - 1);
                    }
                } else if !key.eq(&("Backspace".to_string())) {
                    self.user_input.push(key.chars().nth(0).unwrap());
                }

                true
            }
            ComponentMsg::UpdateSourceText(new_text) => {
                self.source_text = new_text;
                true
            }
            _ => false,
        }
    }
}
