use crate::settings::AppSettings;
use yew::prelude::*;

#[derive(Debug, Clone)]
pub(crate) enum ComponentMsg {
    FontSizeIncreased,
    FontSizeDecreased,
    StateReset,
    IncomingUserInput(String),
}

#[derive(Debug, Clone)]
pub(crate) struct App {
    source_text: String,
    user_input: Vec<char>,
    app_settings: AppSettings,
}

impl Component for App {
    type Message = ComponentMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            source_text: String::from("this is a sample statement"),
            user_input: Vec::new(),
            app_settings: AppSettings::default(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
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
                        </form>

                        // settings
                        <div class={classes!(String::from("mt-2 py-4 flex justify-between items-center space-x-2"))}>
                            // font size
                            <div class={classes!(String::from("w-fit"))}>
                                <h1 class={classes!(String::from("text-sm mb-1 text-base-content font-semibold"))}>{"Font Size"}</h1>
                                <div class={classes!(String::from("divide-x-1 rounded-lg overflow-clip"))}>
                                    <p class={classes!(String::from("h-14 w-20 inline-flex items-center justify-center text-xl bg-red-500"))}>{ self.app_settings.font_size }</p>
                                    <button class={classes!(String::from("settings-button"))} onclick={ctx.link().callback(|_| ComponentMsg::FontSizeIncreased)}>{"+"}</button>
                                    <button class={classes!(String::from("settings-button"))} onclick={ctx.link().callback(|_| ComponentMsg::FontSizeDecreased)}>{"-"}</button>
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

                    <footer class={classes!(String::from("h-10 text-center"))}>
                        <p class={classes!(String::from("text-sm text-base-content"))}>{"Powered By YewRS"}</p>
                    </footer>
                </div>


            </main>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ComponentMsg::FontSizeDecreased => {
                if self.app_settings.font_size > 10 {
                    self.app_settings.dec_font_size();
                    return true;
                }

                false
            }
            ComponentMsg::FontSizeIncreased => {
                if self.app_settings.font_size < 50 {
                    self.app_settings.inc_font_size();
                    return true;
                }

                false
            }
            ComponentMsg::StateReset => {
                self.source_text = String::from("this is a sample statement");
                self.user_input = Vec::new();
                true
            }
            ComponentMsg::IncomingUserInput(key) => {
                if !key.eq(&("Backspace".to_string())) {
                    self.user_input.push(key.chars().nth(0).unwrap());
                } else {
                    if self.user_input.len() > 0 {
                        self.user_input.remove(self.user_input.len() - 1);
                    }
                }
                true
            }
        }
    }
}
