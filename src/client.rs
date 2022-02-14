use hello_world::greeter_client::GreeterClient;
use hello_world::output_client::OutputClient;
use hello_world::HelloRequest;
use hello_world::PrintlnRequest;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    // let response = client.say_hello(request).await?;
    // println!("RESPONSE={:?}", response);


    // Ouput

    let mut client = OutputClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(PrintlnRequest {
        s: "Stuff to be written out by the server".into(),
    });

    for i in (0..10) {
        let the_output = format!("Stuff to be written out by the server: {}", i);
        let request = tonic::Request::new(PrintlnRequest {
            s: the_output,
        });

        let _response = client.println(request).await?;
        // println!("RESPONSE={:?}", response);
    }


    Ok(())
}
