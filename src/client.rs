use hello::say_client::SayClient;
use hello::SayRequest;

pub mod hello {
  tonic::include_proto!("hello");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  // create channel connect to server
  let channel = tonic::transport::Channel::from_static("http://[::1]:50051")
    .connect()
    .await?;

  // create gRPC client from channel
  let mut client = SayClient::new(channel);

  // create new request
  let request = tonic::Request::new(
    SayRequest {
      name: String::from("wilsoncode")
    }
  );

  // send request and wait for response
  let response = client.send(request).await?.into_inner();
  println!("RESPONSE={:?}", response);
  Ok(())
}
