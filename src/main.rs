use chrono::Local;
use std::fs;
use std::path::Path;
use std::fmt;
use anyhow::Result;

mod install;
mod download;
mod cli;
mod remove;

// Error Handler
#[derive(Debug)]
struct UpdateError {
    timestamp: String,
    details: String,
}

impl UpdateError {
    fn new(details: impl Into<String>) -> Self {
        Self {
            timestamp: Local::now().format("%m-%d-%Y-%H:%M:%S").to_string(),
            details: details.into(),
        }
    }
}

impl fmt::Display for UpdateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {}", self.timestamp, self.details)
    }
}

impl std::error::Error for UpdateError {}

fn main() -> Result<()> {
    let args = cli::run()?;

    if args.remove {
        let link_path: Option<&Path> = args.link_path.as_ref().map(|p| p.as_path());
        remove::uninstall_discord(link_path)
            .map_err(|e| anyhow::anyhow!("Uninstall failed: {}", e))?;
        return Ok(());
    }


    check_files_first()
        .map_err(|e| anyhow::anyhow!("OS Check failed: {}", e))?;
    
    perform_update(&args)
        .map_err(|e| anyhow::anyhow!("Update failed: {}", e))?;
    
    Ok(())
}

fn check_file(path: &Path) -> Result<(), UpdateError> {
    if fs::metadata(path).is_err() {
        Err(UpdateError::new(format!("Missing file or dir: {}", path.display())))
    } else {
        //println!("OK: {}", path.display());
        Ok(())
    }
}

// TODO: add check for existing install.
fn check_files_first() -> Result<(), UpdateError> {
    let check_opt: &Path = Path::new("/opt");
    let check_usr: &Path = Path::new("/usr/bin");
    let check_usr_bin: &Path = Path::new("/usr/local/bin");
    check_file(check_opt)?;
    check_file(check_usr)?;
    check_file(check_usr_bin)?;
    Ok(())
}


fn perform_update(args: &cli::Args) -> Result<(), UpdateError>{
    println!("Performing update...");
    download::download_tarfile(args)?;
    install::install_discord()?;
    let link: Option<&Path> = args.link_path.as_ref().map(|p| p.as_path());
    install::post_install(link)?; 
    println!("Update complete!");
    Ok(())
}
// unit tests src/tests.rs
#[cfg(test)]
mod tests;
