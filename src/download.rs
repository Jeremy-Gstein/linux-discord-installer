// download.rs handles HTTP and tar/gz for discord file.
use std::fs::File;
use std::io::Write;
use chrono::Local;
use crate::UpdateError;
use std::io::BufReader;
use flate2::read::GzDecoder;
use tar::Archive;
use crate::cli::Args;


fn extract_tarfile(tarfile: &str) -> Result<(), UpdateError> {
    let file = File::open(tarfile)
        .map_err(|e| UpdateError::new(format!("Failed to open {}: {}", tarfile, e)))?;
    
    let tar_gz = GzDecoder::new(BufReader::new(file));
    let mut archive = Archive::new(tar_gz);
    
    archive.unpack("/tmp")
        .map_err(|e| UpdateError::new(format!("Failed to extract {}: {}", tarfile, e)))?;
    
    println!("Extracted {} to /tmp", tarfile);
    Ok(())
}

pub fn download_tarfile(args: &Args) -> Result<(), UpdateError> {
    let now = Local::now();
    let timestamp = now.format("%Y-%m-%d-%H:%M:%S").to_string();
    let output_file = format!("/tmp/{}-discord.tar.gz", timestamp);
    println!("Downloading Discord from {}", args.url);

    
    let response = reqwest::blocking::get(&args.url)
        .map_err(|e| UpdateError::new(format!("HTTP download failed: {}", e)))?;
    
    if !response.status().is_success() {
        return Err(UpdateError::new(format!("HTTP {} from Discord", response.status())));
    }

    let mut file = File::create(&output_file)
        .map_err(|e| UpdateError::new(format!("Failed to create {}: {}", output_file, e)))?;
    
    let content = response.bytes()
        .map_err(|e| UpdateError::new(format!("Failed to read response: {}", e)))?;
    
    file.write_all(&content)
        .map_err(|e| UpdateError::new(format!("Failed to write {}: {}", output_file, e)))?;
    
    println!("Downloaded to {}", output_file);
    extract_tarfile(&output_file)?;
    Ok(())
}
