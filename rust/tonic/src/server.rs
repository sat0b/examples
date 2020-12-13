use tonic::{transport::Server, Response, Status, Request};

use example::greeter_server::{Greeter, GreeterServer};
use example::{ExampleRequest, ExampleReply};

pub mod example {
    tonic::include_proto!("example");
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<ExampleRequest>
    ) -> Result<Response<ExampleReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = example::ExampleReply {
            message: format!("Hello {}!", request.into_inner().name).into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}