#!/bin/sh

cargo install cargo-generate
cargo install cargo-watch

curr_dir=$(pwd)

mkdir ~/bin
cd ~/bin

# https://github.com/casey/just#packages
curl --proto '=https' --tlsv1.2 -sSf https://just.systems/install.sh | bash -s -- --to ~/bin
export PATH="$PATH:$HOME/bin"
echo 'PATH="$PATH:$HOME/bin"' >> ~/.bashrc

cd $curr_dir
echo "Devcontainer setup finished"