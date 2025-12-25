use clap::Parser;
use crate::UpdateError;

pub const DISCORD_TARBALL: &str = "https://discord.com/api/download?platform=linux&format=tar.gz";

#[derive(Parser, Debug)]
#[command(
    author = "Jeremy-Gstein <jeremy51b5@pm.me>",
    version = "0.1.0",
    about = "Discord Client installer/updater for Linux",
    long_about = "Downloads, extracts, and installs Discord from official tar.gz.\nCreates distro-aware symlink (/usr/bin/discord on Arch, /usr/local/bin/discord on Debian)."
)]
pub struct Args {
    /// Perform an update/install
    #[arg(short, long)]
    pub update: bool,

    /// Uninstall Discord completely
    #[arg(short = 'r', long)]
    pub remove: bool,

    /// Custom symlink target (overrides distro detection)
    #[arg(long, value_parser = clap::value_parser!(std::path::PathBuf))]
    pub link_path: Option<std::path::PathBuf>,

    /// Custom Discord download URL (overrides default)
    #[arg(long, default_value = DISCORD_TARBALL)]
    pub url: String,
}


pub fn run() -> Result<Args, UpdateError> {
    let args = Args::parse();

    let action_count = (args.update as usize) + (args.remove as usize);
    if action_count == 0 {
        return Err(crate::UpdateError::new("No action specified. Use --update or --remove"));
    }
    if action_count > 1 {
        return Err(crate::UpdateError::new("Cannot use both --update and --remove"));
    }
    
    Ok(args)
    
}
