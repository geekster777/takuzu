// #![windows_subsystem = "windows"]

extern crate color_thief;
extern crate image;
extern crate rand;
extern crate sciter;

mod gen_board;

use color_thief::ColorFormat;
use gen_board::{BoardGenerator, BoardState};
use sciter::{dispatch_script_call, Value};

struct Handler {
  archive: sciter::Archive,
}

impl Handler {
  fn color_palette(&self, path: String) -> Value {
    let img_data = self.archive.get(path.as_str()).unwrap();
    let img = image::load_from_memory(img_data).unwrap();
    let pixels = img.as_bytes();
    let palette = color_thief::get_palette(&pixels, ColorFormat::Rgb, 1, 20).unwrap_or_default();

    return palette
      .iter()
      .map(|color| {
        let mut v = Value::new();
        v.set_item("r", i32::from(color.r));
        v.set_item("g", i32::from(color.g));
        v.set_item("b", i32::from(color.b));
        v
      })
      .collect();
  }

  fn gen_takuzu_board(&self, size: i32) -> Value {
    let generator = BoardGenerator {
      size: size as usize,
    };
    let board = generator.gen_board();
    return board
      .into_iter()
      .map(|tile| match tile {
        BoardState::EMPTY => Value::null(),
        BoardState::PRIMARY => Value::from(0),
        BoardState::SECONDARY => Value::from(1),
      })
      .collect();
  }
}

impl sciter::EventHandler for Handler {
  dispatch_script_call! {
    fn color_palette(String);
    fn gen_takuzu_board(i32);
  }
}

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

  let archive = sciter::host::Archive::open(assets).expect("Unable to load archive.");
  let handler = Handler { archive: archive };
  frame.event_handler(handler);
  frame.load_file("this://app/main.htm");
  frame.run_app();
}
