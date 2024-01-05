# 🫂 Each 

simple + secure + sharable cross-node workloads

### Overview

Each will be a binary, runnable on multiple nodes, that can share work securely.

Nodes will communicate with ZMQ, node connections will be predefined via a `confball`, which _will be_ a shared yaml file pointing to the locations of each workers GPG public key. The workers will connect via GPG authetication, then use curve for in transit encryption. 

For starters, keep it simple. If a node is busy, it shares the job state with an adjacent node via ZMQ.

Work is defined by executors. Currently I've got Shell as the only executor type.

Each will expose a gRPC client, capable of receiving work from an outside source.

### Crate Overview

 - `main.rs` is main.
 - `auth` will handle GPG cross-node validation and `confball` interpretation.
 - `comms` will handle ZMQ cross-node communication
 - `executor` will handle job execution per-node. 
 - `provoker` will expose the gRPC server and accept jobs.

### Goals

 - Share task state for parallel execution and horizontal scaling of a long-running task
 - Task state should be serializable somehow and should be passed over the wire to adjacent nodes via zmq
 - Nodes should account and reconcile their differences - ie what they are able to run.

### Initial Testing

 - Send a shell task - `sleep 100` to a bunch of nodes and ensure they distribute.