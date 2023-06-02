fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = prost_build::Config::new();
    config.protoc_arg("--experimental_allow_proto3_optional");
    // Output the generated rs files to `src/proto/`
    tonic_build::configure()
        .out_dir("src/proto/")
        .compile_with_config(config, &["src/proto/aya_cardano.proto"], &["proto"])?;

    Ok(())
}
