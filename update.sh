#!/usr/bin/bash

# Simple bash script to update discord.
# this was the original script used and gave motivation to try a rust approach.

DISCORD_TARBALL="https://discord.com/api/download?platform=linux&format=tar.gz" 

curl -L --proto '=https' --tlsv1.2 -sSf "$DISCORD_TARBALL" --output '/tmp/discord.tar.gz'

tar xzvf /tmp/discord.tar.gz -C /tmp

sudo rm -rf /opt/Discord

sudo mv -f /tmp/Discord /opt/

sudo ln -sf /opt/Discord/Discord /usr/local/bin/discord
