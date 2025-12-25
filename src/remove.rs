use std::path::{Path, PathBuf};
use std::process::Command;
use crate::UpdateError;
use crate::install::default_link_location;

pub fn uninstall_discord(link_path: Option<&Path>) -> Result<(), UpdateError> {
    println!("Uninstalling Discord...");

    // Own the path (either from arg or from default)
    let target_path: PathBuf = match link_path {
        Some(p) => p.to_path_buf(),
        None => default_link_location(),
    };
    let target: &Path = &target_path;

    let status = Command::new("sudo")
        .arg("rm")
        .arg("-f")
        .arg(target)
        .status()
        .map_err(|e| UpdateError::new(format!("Failed to remove symlink: {}", e)))?;

    if status.success() {
        println!("Removed symlink: {}", target.display());
    }

    let status = Command::new("sudo")
        .arg("rm")
        .arg("-rf")
        .arg("/opt/discord")
        .status()
        .map_err(|e| UpdateError::new(format!("Failed to remove /opt/discord: {}", e)))?;

    if status.success() {
        println!("Removed Discord installation");
    }

    println!("Uninstall complete!");
    Ok(())
}

