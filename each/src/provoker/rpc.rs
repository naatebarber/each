use std::error::Error;

pub use each_provoker::provoker_server::{Provoker, ProvokerServer};
use each_provoker::{TaskRequest, TaskResponse};
use tonic::{transport::Server, Request, Response, Status};

use crate::executor::{executor::Executor, shell::ShellExecutor};

pub mod each_provoker {
    tonic::include_proto!("each_provoker");
}

#[derive(Debug, Default)]
pub struct ProvokerService;

impl ProvokerService {
    fn run_executor(
        &self,
        executor: &dyn Executor,
        command: String,
    ) -> Result<String, Box<dyn Error>> {
        let output = executor.exec(command)?;

        Ok(output)
    }

    pub async fn start(rpc_port: u32) {
        let addr = format!("[::1]:{}", rpc_port)
            .parse()
            .expect("Provoker Service Error - Improper address format.");
        let provoker_rpc = ProvokerService::default();

        Server::builder()
            .add_service(ProvokerServer::new(provoker_rpc))
            .serve(addr)
            .await
            .expect(&format!("Failed to start provoker service at {}", rpc_port))
    }
}

#[tonic::async_trait]
impl Provoker for ProvokerService {
    async fn provoke(
        &self,
        request: Request<TaskRequest>,
    ) -> Result<Response<TaskResponse>, Status> {
        let task_request: TaskRequest = request.into_inner();
        let executor_spec = task_request.executor;
        let command = task_request.command;

        println!("RECV ({}) -> ({})", executor_spec, command);

        let executor = match executor_spec.as_str() {
            "shell" => Some(ShellExecutor {}),
            _ => None,
        };

        let reply: TaskResponse;

        if let Some(exe) = executor {
            let result = self.run_executor(&exe, command);
            if let Ok(output) = result {
                reply = TaskResponse {
                    success: true,
                    output: output.into(),
                };
            } else {
                reply = TaskResponse {
                    success: false,
                    output: "Request failed!".into(),
                }
            }
        } else {
            reply = TaskResponse {
                success: false,
                output: format!("Executor '{}' does not exist.", executor_spec),
            };
        }

        Ok(Response::new(reply))
    }
}
