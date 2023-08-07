// site/src/main.rs

// dependencies
use jeff_mitchell_dev_site::app::App;

// main function, sets up logging and renders the App component
fn main() {
    // set up logging
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    console_error_panic_hook::set_once();

    // Render the app component
    yew::Renderer::<App>::new().render();
}
