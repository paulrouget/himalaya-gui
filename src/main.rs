//! FIXME

mod collection_object;
mod task_object;
mod utils;
mod window;

use adw::prelude::{ApplicationExt, ApplicationExtManual, GtkApplicationExt, WidgetExt};
use gtk::gio;
use window::Window;

static APP_ID: &str = "com.paulrouget.himalaya-gui";

fn main() {
  gio::resources_register_include!("himalaya-gui.gresource").expect("Failed to register resources.");

  // Create a new application
  let app = adw::Application::builder().application_id(APP_ID).build();

  // Connect to signals
  app.connect_startup(setup_shortcuts);
  app.connect_activate(build_ui);

  // Run the application
  app.run();
}

fn setup_shortcuts(app: &adw::Application) {
  app.set_accels_for_action("win.filter('All')", &["<Ctrl>a"]);
  app.set_accels_for_action("win.filter('Open')", &["<Ctrl>o"]);
  app.set_accels_for_action("win.filter('Done')", &["<Ctrl>d"]);
}

fn build_ui(app: &adw::Application) {
  // Create a new custom window and show it
  let window = Window::new(app);
  window.show();
}
