use greeter::greeter_client::GreeterClient;
use greeter::HelloRequest;
use std::io::Read;
use tar::Builder;

use tempfile::NamedTempFile;

pub mod greeter {
    tonic::include_proto!("greeter");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("https://[::1]:50051").await?;

    let file = NamedTempFile::new()?;
    let mut a = Builder::new(&file);

    a.append_path("myfile.txt").unwrap();

    // Get a new file handler to the tar file.
    let mut fsend = file.reopen()?;
    let mut buffer = Vec::new();

    // read the whole file
    fsend.read_to_end(&mut buffer)?;

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
        file: buffer,
    });

    println!("Sending request to gRPC Server...");
    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
