use tonic::{transport::Server, Request, Response, Status};
use hello::say_server::{Say, SayServer};
use hello::{SayResponse, SayRequest};

pub mod hello {
  tonic::include_proto!("hello");
}

// define struct for service
#[derive(Default)]
pub struct MySay {}

// implement rpc for service define in .proto
#[tonic::async_trait]
impl Say for MySay {
  // rpc implemented as function
  async fn send(&self, request: Request<SayRequest>) -> Result<Response<SayResponse>, Status> {
    // return response as SayResponse message defined in .proto
    Ok(Response::new(SayResponse {
      // read data from request which is a wrapper around SayRequest message defined in .proto
      message: format!("hello {}", request.get_ref().name)
    }))
  }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  // define address for our service
  let addr = "[::1]:50051".parse().unwrap();
  //create service
  let say = MySay::default();
  println!("Server listening on {}", addr);
  // add service to server
  Server::builder()
    .add_service(SayServer::new(say))
    .serve(addr)
    .await?;
  Ok(())
}
