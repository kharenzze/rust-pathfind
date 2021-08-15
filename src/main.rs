use simple_logger::{SimpleLogger};
use log::{LevelFilter, info};
use pathfind::game::MainState;

fn main() {
  SimpleLogger::new()
    .with_colors(true)
    .with_level(LevelFilter::Error)
    .with_module_level("pathfind", LevelFilter::Debug)
    .init()
    .unwrap();

  let state = MainState{};

  info!("Hello, world!");
}
