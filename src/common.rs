use anyhow::{Context, Result};
use std::{env, path::PathBuf};
pub fn access_archive_path(filename: &str) -> Result<PathBuf> {
    let mut file_path = env::home_dir().context("Failed to get home directory!")?;
    file_path.push("slime_archive");
    file_path.push(filename);
    Ok(file_path)
}
