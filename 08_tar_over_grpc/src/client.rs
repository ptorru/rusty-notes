use greeter::greeter_client::GreeterClient;
use greeter::HelloRequest;

pub mod greeter {
    tonic::include_proto!("greeter");
}

pub fn file_reader(path: &str) -> Vec<u8> {
    println!("{}",  path);
    vec![1u8, 2, 3]
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("https://[::1]:50051").await?;

    let path = "myfile.txt";

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
        file: file_reader(path),
    });

    println!("Sending request to gRPC Server...");
    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
