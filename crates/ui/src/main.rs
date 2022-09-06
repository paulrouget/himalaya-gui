// hide console window on Windows in release:
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(result_option_inspect)]

mod app;
mod server;
mod theme;
mod ui;

fn main() {
  pretty_env_logger::init();

  let options = eframe::NativeOptions {
    // Generally, looks better. But NSVisualEffectView doesn't work.
    // Also fixes https://github.com/emilk/egui/issues/903
    renderer: eframe::Renderer::Wgpu,
    // We do that ourself:
    follow_system_theme: false,
    // On Mac, window controls and titlebar will overlap the content
    fullsize_content: true, //
    // Necessary for NSVisualEffectView
    transparent: true,
    ..Default::default()
  };

  eframe::run_native(
    "Himalaya",
    options,
    Box::new(|cc| {
      match app::App::new(cc) {
        Ok(app) => Box::new(app),
        Err(e) => {
          eprintln!("Error: {}", e);
          // Only allowed here.
          #[allow(clippy::exit)]
          std::process::exit(1);
        },
      }
    }),
  );
}
