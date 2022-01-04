#![windows_subsystem = "windows"]

extern crate sciter;

fn main() {
  let assets = include_bytes!("../target/assets.rc");

  // Enable debug mode for all windows, so that we can inspect them via Inspector.
  sciter::set_options(sciter::RuntimeOptions::DebugMode(true)).unwrap();

  let mut frame = sciter::window::Builder::main_window()
    .with_size((800, 800))
    .with_pos((300, 100))
    .resizeable()
    .glassy()
    .create();

  frame
    .set_options(sciter::window::Options::TransparentWindow(true))
    .unwrap();
  frame
    .archive_handler(assets)
    .expect("Unable to load archive");

  frame.set_title("Takuzu - By Nick Hilton");

  frame.load_file("this://app/main.htm");
  frame.run_app();
}
