use tonic::{transport::Server, Request, Response, Status};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::output_server::{Output, OutputServer};

use hello_world::{HelloReply, HelloRequest, PrintlnRequest, PrintlnReply};

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }
}

#[derive(Default)]
pub struct MyOutput {}

#[tonic::async_trait]
impl Output for MyOutput {

    async fn println(&self, request: Request<PrintlnRequest>) -> Result<Response<PrintlnReply>, Status> {
        // println!("Got a request from {:?}", request.remote_addr());

        // println!("\nactual output:");
        println!("{}", request.into_inner().s);

        let reply = hello_world::PrintlnReply {
            // message: format!("Hello {}!", requet.into_inner().name),
        };
        Ok(Response::new(reply))
    }
    
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let greeter = MyGreeter::default();
    let output = MyOutput::default();

    println!("GreeterServer listening on {}", addr);
    println!("OutputServer listening on {}", addr);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .add_service(OutputServer::new(output))
        .serve(addr)
        .await?;

    Ok(())
}
