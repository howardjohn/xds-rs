use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;
use xds::DiscoveryRequest;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

pub mod xds {
    tonic::include_proto!("envoy.service.discovery.v3");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    // let request = tonic::Request::new(HelloRequest {
    //     name: "Tonic".into(),
    // });

    let xds = tonic::Request::new(DiscoveryRequest{});

    // let response = client.say_hello(request).await?;

    // println!("RESPONSE={:?}", response);
    println!("XDS={:?}", xds);

    Ok(())
}
