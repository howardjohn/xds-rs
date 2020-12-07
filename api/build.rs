fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        .compile(
            &[
                // "proto/envoy/service/discovery/v3/discovery.proto",
                "proto/envoy/config/core/v3/base.proto",
            ],
            &["proto/"],
        )?;
    Ok(())
}
