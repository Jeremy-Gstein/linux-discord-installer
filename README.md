## Installer for Discord Client on Linux:
for 99.9% of users, your distros package manager will have maintainers ontop of this and you can `apt install discord`, `apk add discord`, `pacman -S discord`, etc..

For a quick solution, this cli util makes installing, removing, updating, and verifying discord client installs simple for those moments you NEED discord but cant get an update package from your distro. 

## Check Releases for latest version! 
to verify install:
```bash
# (clone repo and cd to repo first)
./verify_download.sh --verify
```
to install a static binary to /tmp
```bash
# this will do --verify before installing static binary to a known location (/tmp).
./verify_download.sh --install 
/tmp/linux-discord-installer-x86_64-musl --help
```

## Complile/Build yourself:
```bash
REPO="https://github.com/Jeremy-Gstein/linux-discord-installer"
git clone $REPO
cd $REPO 

# Install rust, musl (needed for static build only)
rustup target add x86_64-unknown-linux-musl
# Dynamic bin (gcc)
cargo build --release
# Static bin (musl)
cargo build --release --target x86_64-unknown-linux-musl
# Update Discord With Cargo
cargo run -- --update
```


