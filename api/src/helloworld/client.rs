
// https://github.com/cetanu/sovereign-rs/blob/master/src/proto.rs

pub mod google {
    pub mod rpc {
        tonic::include_proto!("google.rpc");
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    // let request = tonic::Request::new(HelloRequest {
    //     name: "Tonic".into(),
    // });

    // let xds = tonic::Request::new(DiscoveryRequest{});

    // let response = client.say_hello(request).await?;

    // println!("RESPONSE={:?}", response);
    // println!("XDS={:?}", xds);

    Ok(())
}
