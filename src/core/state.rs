use ggez::event::EventHandler;
use ggez::error::{GameResult, GameError};
pub struct MainState {}

impl EventHandler<GameError> for MainState {
  fn update(&mut self, _ctx: &mut ggez::Context) -> GameResult {
    Ok(())
  }

  fn draw(&mut self, _ctx: &mut ggez::Context) -> GameResult {
    Ok(())
  }
}