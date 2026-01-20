fn main() {
    divan::main();
}

use anyhow::{anyhow, Context, Result};
use std::io;

#[divan::bench]
fn create_simple_error() -> anyhow::Error {
    anyhow!("something went wrong")
}

#[divan::bench]
fn create_error_with_interpolation() -> anyhow::Error {
    let value = 42;
    anyhow!("error processing value: {}", value)
}

#[divan::bench]
fn error_from_std_error() -> anyhow::Error {
    let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
    anyhow::Error::from(io_err)
}

#[divan::bench]
fn add_context_static() -> Result<()> {
    Err(io::Error::new(io::ErrorKind::NotFound, "file not found"))
        .context("Failed to read config file")
}

#[divan::bench]
fn add_context_dynamic() -> Result<()> {
    let path = "/path/to/file.txt";
    Err(io::Error::new(io::ErrorKind::NotFound, "file not found"))
        .with_context(|| format!("Failed to read file at {}", path))
}

#[divan::bench]
fn chain_multiple_contexts() -> Result<()> {
    Err(io::Error::new(io::ErrorKind::NotFound, "file not found"))
        .context("Failed to read config")
        .context("Failed to initialize application")
        .context("Startup failed")
}

#[divan::bench]
fn downcast_error() {
    let err = anyhow::Error::from(io::Error::new(io::ErrorKind::NotFound, "file not found"));
    let _ = err.downcast_ref::<io::Error>();
}

#[divan::bench]
fn format_error_display() -> String {
    let err = anyhow!("something went wrong");
    format!("{}", err)
}

#[divan::bench]
fn format_error_debug() -> String {
    let err = anyhow!("something went wrong");
    format!("{:?}", err)
}

#[divan::bench]
fn iterate_error_chain() -> usize {
    let err = Err::<(), _>(io::Error::new(io::ErrorKind::NotFound, "file not found"))
        .context("Failed to read config")
        .context("Failed to initialize")
        .unwrap_err();
    
    err.chain().count()
}
