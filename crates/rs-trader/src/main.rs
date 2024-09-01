use gui::App;
use plugins_interfaces::PluginRegistrar;

const TITLE: &str = "rsTrader";

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let mut registrar = PluginRegistrar::default();
    let mut exe_dir = std::env::current_exe().unwrap();
    exe_dir.pop();
    registrar.load_plugins(exe_dir);

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        TITLE,
        native_options,
        Box::new(|cc| Ok(Box::new(<App>::new(cc, registrar)))),
    )
}
