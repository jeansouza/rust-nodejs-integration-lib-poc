extern crate napi_build;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  napi_build::setup();
  tonic_build::compile_protos("proto/test.proto")?;  
  Ok(())
}
