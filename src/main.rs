use simple_logger::{SimpleLogger};
use log::{LevelFilter, info};

fn main() {
  SimpleLogger::new()
    .with_colors(true)
    .with_level(LevelFilter::Error)
    .with_module_level("pathfind", LevelFilter::Debug)
    .init()
    .unwrap();

  info!("Hello, world!");
}
