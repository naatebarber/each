use super::executor::Executor;
use std::error::Error;
use std::process::Command;

pub struct ShellExecutor;

impl Executor for ShellExecutor {
    fn exec(&self, cmd: String) -> Result<String, Box<dyn Error>> {
        let mut args = cmd.split_whitespace().map(|x| String::from(x));
        let program = args
            .next()
            .expect("Shell executor cannot run an empty command.");

        let result = Command::new(program)
            .args(args.collect::<Vec<String>>())
            .output()?;

        let stdout_bytes = result.stdout.as_slice().to_vec();
        let stdout = String::from_utf8(stdout_bytes)?;

        Ok(stdout)
    }
}
