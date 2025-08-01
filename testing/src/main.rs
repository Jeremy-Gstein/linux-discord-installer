use chrono::Local;
use std::fs::File;
use std::io::{self, Write};
use reqwest::blocking::get;  
use std::process::Command;

fn download_tarfile() -> Result<(), Box<dyn std::error::Error>> {
    // Timestamp
    let now = Local::now();
    let timestamp = now.format("%Y-%m-%d-%H:%M:%S").to_string();
    let url = "https://discord.com/api/download?platform=linux&format=tar.gz";
    let tmp_path = "/tmp/";
    let output_file = format!("{}{}-discord.tar.gz", tmp_path, timestamp);
    // Download the file using reqwest
    let mut response = get(url)?;
    // Check if the response is successful
    if !response.status().is_success() {
        return Err(Box::new(io::Error::new(io::ErrorKind::Other, "Failed to download file")));
    }
    // Create a file to save the downloaded content
    let mut output = File::create(&output_file)?;
    // Copy the response body to the file
    io::copy(&mut response, &mut output)?;
    println!("Downloaded: {}", &output_file);
    // Extract the tarball 
    extract_tarfile(&output_file)?;
    Ok(())
}


fn extract_tarfile(tarfile: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Extract tar.gz to /tmp
    let status = Command::new("tar")
        .args(&["xzvf", &tarfile , "-C", "/tmp"])
        .status()?;

    if status.success() {
        println!("Extracted: {}", tarfile);
    } else {
        eprintln!("Failed to extract tarfile: {}", tarfile);
    }

    Ok(())
}

fn main() {
    if let Err(e) = download_tarfile() {
        eprintln!("Error: {}", e);
    }
}
