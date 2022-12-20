use anyhow::{Context, Result};
use std::fs::File;
use thiserror::Error;

#[derive(Debug, Error)]
enum ApiError {
    #[error("InternalServerError: {0}")]
    InternalServerError(String),
    #[error("NotFound")]
    NotFound,
}

fn fetch_api() -> Result<()> {
    Err(ApiError::InternalServerError("[always_my_error]".to_string()).into())
}

fn maybe_fail() -> Result<()> {
    let _r = fetch_api()?;
    let filename = "foo.txt";
    let _f = File::open(filename).context(format!("failed to open file: {}", filename))?;
    Ok(())
}

pub fn run() -> Result<()> {
    let _l = maybe_fail()?;
    Ok(())
}
