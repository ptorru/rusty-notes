use greeter::greeter_client::GreeterClient;
use greeter::HelloRequest;
use std::io::{Read};
use std::fs::File;

pub mod greeter {
    tonic::include_proto!("greeter");
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("https://[::1]:50051").await?;

    let mut f = File::open("src/myfile.txt")?;
    let mut buffer = Vec::new();

    // read the whole file
    f.read_to_end(&mut buffer)?;

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
        file: buffer,
    });

    println!("Sending request to gRPC Server...");
    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
