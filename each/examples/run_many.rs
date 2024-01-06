use each::provoker::rpc::ProvokerService;
use tokio;

#[tokio::main]
async fn main() {
    println!("Running Each on (3) threads, simulating three machines maybe.");

    let t1 = tokio::spawn(run_at_port(5001));
    let t2 = tokio::spawn(run_at_port(5002));
    let t3 = tokio::spawn(run_at_port(5003));

    let threads = tokio::join!(t1, t2, t3);
    threads.0.unwrap();
    threads.1.unwrap();
    threads.2.unwrap();
}

async fn run_at_port(port: u32) {
    println!("Running Each at port {}", port);
    ProvokerService::start(port).await;
}
