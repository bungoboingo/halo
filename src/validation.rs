use naga::valid::Capabilities;

//assumes shader is wgsl
pub fn validate(shader: &str) -> Result<(), Error> {
    //parse separately so we can show errors instead of panicking on pipeline creation
    let parsed =
        naga::front::wgsl::parse_str(shader).map_err(|err| Error::Parse(err.to_string()))?;

    naga::valid::Validator::new(
        naga::valid::ValidationFlags::default(),
        Capabilities::all(), //TODO get from device capabilities
    )
    .validate(&parsed)
    .map_err(|err| Error::Validation(err.to_string()))?;

    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Shader parsing error: {0}")]
    Parse(String),
    #[error("Validation error: {0}")]
    Validation(String),
}
