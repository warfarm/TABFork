fn main() -> Result<(), std::io::Error> {
    prost_build::compile_protos(&["gtfsproto/gtfs-realtime.proto"], &["gtfsproto"])?;
    slint_build::compile("ui/TransitUi.slint").unwrap();
    Ok(())
}
