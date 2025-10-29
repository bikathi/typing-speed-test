use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <main class={classes!(String::from("flex justify-center h-screen bg-base-100"))}>
            <div class={classes!(String::from("flex flex-col justify-between h-full w-full md:w-1/2"))}>
                <div>/* */</div>

                <div class={classes!(String::from("h-fit"))}>
                    // allow users to upload their own source file
                    <form class={classes!(String::from("flex items-center justify-between"))}>
                        <span class={classes!(String::from("text-4xl text-base-content"))}>
                            {"05:00"}
                        </span>
                        <button class={classes!(String::from("py-2 px-3 h-14 rounded-md font-semibold text-sm outline-none bg-secondary text-base-content"))}>
                            {"Custom Text File"}
                        </button>
                    </form>

                    // settings
                    <div class={classes!(String::from("mt-2 py-4 flex justify-between items-center space-x-2"))}>
                        // font size
                        <div class={classes!(String::from("w-fit"))}>
                            <h1 class={classes!(String::from("text-sm mb-1 text-base-content font-semibold"))}>{"Font Size"}</h1>
                            <div class={classes!(String::from("divide-x-1 rounded-lg overflow-clip"))}>
                                <p class={classes!(String::from("h-14 w-20 inline-flex items-center justify-center text-xl bg-red-500"))}>{"20"}</p>
                                <button class={classes!(String::from("settings-button"))}>{"+"}</button>
                                <button class={classes!(String::from("settings-button"))}>{"-"}</button>
                            </div>
                        </div>

                        // theme
                        <div class={classes!(String::from("w-fit"))}>
                            <h1 class={classes!(String::from("text-sm mb-1 text-base-content font-semibold"))}>{"Theme"}</h1>
                            <div class={classes!(String::from("divide-x-1 rounded-lg overflow-clip"))}>
                                <button class={classes!(String::from("settings-button"))}>{"+"}</button>
                                <button class={classes!(String::from("settings-button"))}>{"-"}</button>
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
                            <button class={classes!(String::from("settings-button rounded-full"))}>{"+"}</button>
                        </div>
                    </div>

                    // typing input
                    <form class={classes!(String::from("w-full h-96 mt-5 relative"))}>
                        <textarea class={classes!(String::from("text-base-content/50 z-10 input-areas"))} defaultvalue="this is a sample statement" />
                        <textarea class={classes!(String::from("text-base-content z-20 bg-transparent input-areas"))} />
                    </form>

                    // reset button
                    <div class={classes!(String::from("mt-5 text-center h-fit"))}>
                        <button class={classes!(String::from("settings-button rounded-full"))} title="Reset">{"+"}</button>
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
