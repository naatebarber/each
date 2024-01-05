use each_provoker::provoker_client::ProvokerClient;
use each_provoker::TaskRequest;

pub mod each_provoker {
    tonic::include_proto!("each_provoker");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ProvokerClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(TaskRequest {
        executor: "shell".into(),
        command: "echo yousocool".into(),
    });

    let response = client.provoke(request).await?;

    println!("RECV {:?}", response);

    Ok(())
}
