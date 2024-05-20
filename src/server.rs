use tonic::{Request, Response, Status};
use tonic::transport::Server;
use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloResponse, HelloRequest};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(&self, request: Request<HelloRequest>) -> Result<Response<HelloResponse>, Status> {

        let reply = HelloResponse {
            message: format!("Hello {} from Server Side", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "[::1]:50051".parse()?;
    let greeter_service = MyGreeter::default();
    println!("Listening...!");
    Server::builder()
        .add_service(GreeterServer::new(greeter_service))
        .serve(address)
        .await?;

    Ok(())
}
