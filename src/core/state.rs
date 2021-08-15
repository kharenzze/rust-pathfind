use ggez::error::{GameError, GameResult};
use ggez::event::EventHandler;
use ggez::graphics::Color;
use ggez::*;
use log::*;
pub struct MainState {}

impl EventHandler<GameError> for MainState {
  fn update(&mut self, ctx: &mut ggez::Context) -> GameResult {
    const TARGET_FPS: u32 = 60;
    while timer::check_update_time(ctx, TARGET_FPS) {
      let dt = timer::delta(ctx);
      let ms = dt.as_secs_f64() * 1000.0;
      let diff = 100.0 / 6.0 - ms;
      debug!("dt {}, diff {}", ms, diff);
    }
    Ok(())
  }

  fn draw(&mut self, ctx: &mut ggez::Context) -> GameResult {
    graphics::clear(ctx, Color::BLACK);
    graphics::present(ctx)?;
    let dt = timer::delta(ctx);
    let ms = dt.as_secs_f64() * 1000.0;
    debug!("dtt {}", ms);
    timer::yield_now();
    Ok(())
  }
}
