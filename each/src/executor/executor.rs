use std::error::Error;

pub trait Executor {
  fn exec(cmd: String) -> Result<String, Box<dyn Error>>;
}