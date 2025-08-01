use clap::Parser;
use std::process::Command;
use chrono::Local;

/// A simple program with an update flag
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Perform an update
    #[arg(long)]
    update: bool,
}

// Discord tar.gz link for discorc client Linux install
const DISCORD_TARBALL: &str = &"https://discord.com/api/download?platform=linux&format=tar.gz";


fn main() {
    let args = Args::parse();
    if args.update {
        perform_update();
    } else {
        println!("No update flag provided. Run with --update to perform update.");
    }
}

fn install_discord() {
    // Remove existing /opt/Discord (with sudo)
    // NOTE: this assumes the os already has a /opt dir 
    // TODO: check if exists and skip if not instead of assume fail here.
    let status = Command::new("sudo")
        .args(&["rm", "-rf", "/opt/Discord"])
        .status()
        .expect("Failed to remove old Discord");
    assert!(status.success());

    // Move the new folder into /opt/
    // NOTE: this assumes the os already has a /opt dir 
    // TODO: use a less intrusive directory that does not need sudo.
    let status = Command::new("sudo")
        .args(&["mv", "-f", "/tmp/Discord", "/opt/"])
        .status()
        .expect("Failed to move Discord to /opt/");
    assert!(status.success());


}

fn post_install() {
    // Create/overwrite symlink
    // this lets us 'source' local config to new discord binary
    // Explaination:
    //  - if you signed in before, you likley wont need to login again
    let status = Command::new("sudo")
        .args(&["ln", "-sf", "/opt/Discord/Discord", "/usr/local/bin/discord"])
        .status()
        .expect("Failed to create symlink");
    assert!(status.success());
}


fn download_tarfile() {
    // timestamp
    let now = Local::now();
    let timestamp = now.format("%Y-%m-%d-%H:%M:%S").to_string();

    let url = &DISCORD_TARBALL;
    let tmp_path: &str = &"/tmp/";
    let output_file = format!("{}{}-discord.tar.gz", tmp_path, timestamp);

    let status = Command::new("curl")
        .args(&["-L", "-o", &output_file, url])
        .status()
        .expect("failed to execute curl");

    if status.success() {
        println!("Downloaded {}", &output_file);
        extract_tarfile(&output_file);
    } else {
        eprintln!("curl failed with status: {}", status);
    }

}

fn extract_tarfile(tarfile: &str) {
    // Extract tar.gz to /tmp
    let status = Command::new("tar")
        .args(&["xzvf", &tarfile , "-C", "/tmp"])
        .status()
        .expect("Failed to run tar. Unable to Extract Archive...");
    assert!(status.success());
}

fn perform_update() {
    println!("Performing update...");
    download_tarfile();
    install_discord();
    post_install();
    println!("Update complete!");
}
