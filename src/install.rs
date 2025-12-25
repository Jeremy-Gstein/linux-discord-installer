use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use crate::UpdateError;

pub fn install_discord() -> Result<(), UpdateError> {
    let status = Command::new("sudo")
        .args(&["rm", "-rf", "/opt/discord"])
        .status()
        .map_err(|e| UpdateError::new(format!("Failed to remove old Discord: {}", e)))?;
    
    if !status.success() {
        return Err(UpdateError::new("rm command failed"));
    }

    let status = Command::new("sudo")
        .args(&["mv", "-f", "/tmp/Discord", "/opt/discord"])
        .status()
        .map_err(|e| UpdateError::new(format!("Failed to move Discord: {}", e)))?;
    
    if !status.success() {
        return Err(UpdateError::new("mv command failed"));
    }

    let status = Command::new("sudo")
        .args(&["chown", "-R", "root:root", "/opt/discord"])
        .status()
        .map_err(|e| UpdateError::new(format!("Failed to change owner: {}", e)))?;
    
    if !status.success() {
        return Err(UpdateError::new("Failed to change owner of /opt/discord.."));
    }


    Ok(())
}

/// post_install takes a specified target path to install (symlink) the discord binary
pub fn post_install(target: Option<&Path>) -> Result<(), UpdateError> {
    let source = Path::new("/opt/discord/Discord");
    let target = match target {
        Some(p) => p.to_path_buf(),
        None => default_link_location(),
    };

    let status = Command::new("sudo")
        .args([
            "ln",
            "-sf",
            source.to_str().expect("source path not valid UTF-8"),
            target.to_str().expect("target path not valid UTF-8"),
        ])
        .status()
        .map_err(|e| UpdateError::new(format!("Failed to spawn sudo ln: {}", e)))?;

    if !status.success() {
        return Err(UpdateError::new(format!("Symlink creation failed (sudo ln exited with {})", status)));
    }

    println!("Created symlink: {} -> {}", target.display(), source.display());
    Ok(())
}

/// default_link_location enumerates known distro PATHs for packages.
/// TODO: Add more distros and test cases by using container images.
pub fn default_link_location() -> PathBuf {
    match detect_distro() {
        Distro::Arch => PathBuf::from("/usr/bin/discord"),
        Distro::DebianLike => PathBuf::from("/usr/local/bin/discord"),
        Distro::Other => PathBuf::from("/usr/bin/discord"),
    }
}

#[derive(Debug)]
pub enum Distro {
    Arch,
    DebianLike,
    Other,
}

pub fn detect_distro() -> Distro {
    let os_release = fs::read_to_string("/etc/os-release").unwrap_or_default();

    let id = os_release
        .lines()
        .find(|l| l.starts_with("ID="))
        .and_then(|l| l.split_once('=').map(|(_, v)| v.trim_matches('"')))
        .unwrap_or("");

    let id_like = os_release
        .lines()
        .find(|l| l.starts_with("ID_LIKE="))
        .and_then(|l| l.split_once('=').map(|(_, v)| v.trim_matches('"')))
        .unwrap_or("");

    if id == "arch" || id_like.contains("arch") {
        Distro::Arch
    } else if id == "debian"
        || id == "ubuntu"
        || id_like.contains("debian")
        || id_like.contains("ubuntu")
    {
        Distro::DebianLike
    } else {
        Distro::Other
    }
}

