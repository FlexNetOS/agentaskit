use std::env;

fn main() {
    // Set build time for version tracking with UTC timestamp
    let build_time = std::env::var("BUILD_TIME").unwrap_or_else(|_| {
        chrono::Utc::now().to_rfc3339()
    });
    println!("cargo:rustc-env=BUILD_TIME={}", build_time);

    // Generate protobuf files if they exist
    if std::path::Path::new("proto").exists() {
        tonic_build::configure()
            .build_server(true)
            .build_client(true)
            .compile(&["proto/agentaskit.proto"], &["proto"])
            .unwrap_or_else(|e| panic!("Failed to compile protos: {}", e));
    }
}
