fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/scraper_rpc.proto")?;
    tonic_build::compile_protos("proto/prediction.proto")?;
    tonic_build::compile_protos("proto/scoring.proto")?;
    tonic_build::compile_protos("proto/web-server.proto")?;
    Ok(())
}
