use simple_logger::{SimpleLogger};
use glam::*;
use log::{LevelFilter, info};
use ggez::*;
use pathfind::game::MainState;

const RES_1080: Vec2 = const_vec2!([1920.0, 1080.0]);

fn main() {
  SimpleLogger::new()
    .with_colors(true)
    .with_level(LevelFilter::Error)
    .with_module_level("pathfind", LevelFilter::Debug)
    .init()
    .unwrap();

  info!("Hello, world!");
  let window_setup = ggez::conf::WindowSetup::default().title("Steering");
  let res = RES_1080;
  let window_mode = ggez::conf::WindowMode::default()
    .min_dimensions(res.x, res.y)
    .dimensions(res.x, res.y);
  // Make a Context.
  let (mut ctx, event_loop) = ContextBuilder::new("Steering", "Kharenzze")
    .window_setup(window_setup)
    .window_mode(window_mode)
    .build()
    .expect("aieee, could not create ggez context!");

  let mut state = MainState{};

  event::run(ctx, event_loop, state);
}
