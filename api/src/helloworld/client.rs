use prost::Message;
use xds::aggregated_discovery_service_client::AggregatedDiscoveryServiceClient;

use futures::stream;

pub mod xds {
    tonic::include_proto!("envoy.service.discovery.v3");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = AggregatedDiscoveryServiceClient::connect("http://[::1]:15010").await?;

    let outbound = stream::iter(vec![xds::DiscoveryRequest {
        type_url: "type.googleapis.com/envoy.config.cluster.v3.Cluster".into(),
        node: Some(xds::Node {
            id: "sidecar~1.1.1.1~debug.default~default.svc.cluster.local".into(),
            ..Default::default()
        }),
        ..Default::default()
    }]);

    let response = client
        .stream_aggregated_resources(tonic::Request::new(outbound))
        .await?;
    let mut inbound = response.into_inner();

    while let Some(resp) = inbound.message().await? {
        for resource in resp.resources {
            let r = xds::Resource::decode(resource.value.as_slice())?;
            println!("RESPONSE = {:?}", r.name);
        }
    }

    Ok(())
}
