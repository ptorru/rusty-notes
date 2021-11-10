use tonic::{transport::Server, Request, Response, Status};

use greeter::greeter_server::{Greeter, GreeterServer};
use greeter::{HelloResponse, HelloRequest};

use tempfile::tempfile;
use tempfile::NamedTempFile;
use std::io::{self, Write, Read};

// Import the generated proto-rust file into a module
pub mod greeter {
    tonic::include_proto!("greeter");
}

// Implement the service skeleton for the "Greeter" service
// defined in the proto
#[derive(Debug, Default)]
pub struct MyGreeter {}

// Implement the service function(s) defined in the proto
// for the Greeter service (SayHello...)
#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        println!("Received request from: {:?}", request);

        let packet = request.into_inner();

        // Create a file inside of `std::env::temp_dir()`.
        let mut file1 = NamedTempFile::new()?;

        // Write some test data to the first handle.
        file1.write_all(&packet.file)?;

        // Re-open it.
        let mut file2 = file1.reopen()?;

        // Read the test data using the second handle.
        let mut buf = String::new();
        file2.read_to_string(&mut buf)?;

        println!("{}", buf);

        let response = greeter::HelloResponse {
            message: format!("Hello {}!", packet.name).into(),
        };

        Ok(Response::new(response))
    }
}

// Runtime to run our server
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    println!("Starting gRPC Server...");
    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
