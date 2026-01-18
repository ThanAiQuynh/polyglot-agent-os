fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../../contracts/learning_service.proto")?;
    Ok(())
}
