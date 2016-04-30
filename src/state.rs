use std::time::Duration;

use GameError;

pub trait State {
    fn new<T: State>() -> T;
    fn load(&mut self) -> Result<(), GameError>;
    fn update(&mut self, dt: Duration) -> Result<(), GameError>;
    fn draw(&self) -> Result<(), GameError>;
}
