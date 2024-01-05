use std::error::Error;

pub trait Executor {
    fn exec(&self, cmd: String) -> Result<String, Box<dyn Error>>;
}
