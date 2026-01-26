use anyhow::{anyhow, Context, Error, Result};
use std::io;

fn main() {
    divan::main();
}

#[divan::bench]
fn error_creation_anyhow_macro() -> Error {
    anyhow!("An error occurred")
}

#[divan::bench]
fn error_creation_with_format() -> Error {
    anyhow!("Error with value: {}", 42)
}

#[divan::bench]
fn error_from_io_error() -> Error {
    io::Error::new(io::ErrorKind::NotFound, "file not found").into()
}

#[divan::bench]
fn error_with_context() -> Result<()> {
    Err(io::Error::new(io::ErrorKind::NotFound, "file not found"))
        .context("Failed to read configuration file")
}

#[divan::bench]
fn error_with_context_closure() -> Result<()> {
    let filename = "config.json";
    Err(io::Error::new(io::ErrorKind::NotFound, "file not found"))
        .with_context(|| format!("Failed to read file: {}", filename))
}

#[divan::bench]
fn error_downcast() {
    let error: Error = io::Error::new(io::ErrorKind::NotFound, "file not found").into();
    divan::black_box(error.downcast_ref::<io::Error>());
}

#[divan::bench]
fn error_chain_iteration() {
    let base_error = io::Error::new(io::ErrorKind::NotFound, "file not found");
    let result: Result<()> = Err(base_error).context("Failed to read configuration");
    
    if let Err(error) = result {
        for cause in error.chain() {
            divan::black_box(cause);
        }
    }
}

#[divan::bench]
fn error_display() -> String {
    let error = anyhow!("An error occurred");
    error.to_string()
}

#[divan::bench]
fn error_debug() -> String {
    let error = anyhow!("An error occurred");
    format!("{:?}", error)
}
