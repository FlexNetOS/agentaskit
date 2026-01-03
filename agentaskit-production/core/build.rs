fn main() {
    // Set build time for version tracking
    println!("cargo:rustc-env=BUILD_TIME={}", std::env::var("BUILD_TIME").unwrap_or_else(|_| "unknown".to_string()));

    // Generate protobuf files if they exist
    if std::path::Path::new("proto").exists() {
        // Collect all .proto files in the "proto" directory
        let proto_files: Vec<String> = std::fs::read_dir("proto")
            .unwrap_or_else(|e| panic!("Failed to read proto directory: {}", e))
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                if path.extension().and_then(|ext| ext.to_str()) == Some("proto") {
                    Some(path.to_string_lossy().into_owned())
                } else {
                    None
                }
            })
            .collect();
        let includes: Vec<String> = vec!["proto".to_string()];
        tonic_build::configure()
            .build_server(true)
            .build_client(true)
            .compile(&proto_files, &includes)
            .unwrap_or_else(|e| panic!("Failed to compile protos: {}", e));
    }
}