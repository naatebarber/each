use crate::provoker::rpc::ProvokerService;
use tokio;

pub struct Each {
    port: u32,
}

impl Each {
    pub fn new(port: u32) -> Self {
        Each { port }
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let provoker = tokio::spawn(ProvokerService::start(self.port + 1));

        let (provoker_task,) = tokio::join!(provoker);
        provoker_task.expect("Failed to start Provoker service.");

        Ok(())
    }
}
