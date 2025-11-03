mod app;
mod settings;

fn main() {
    // init logger
    wasm_logger::init(wasm_logger::Config::default());

    // render main page
    yew::Renderer::<app::App>::new().render();
}
