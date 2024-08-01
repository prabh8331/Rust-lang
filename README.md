# Rust-lang
Getting started with the Rust
1. setting up the git with the github
git init
git add README.md
git commit -m "first commit"
git config --global user.name "prabh8331"
git config --global user.email "prabh8331@gmail.com"


eval "$(ssh-agent -s)"
ssh-add ~/.ssh/id_ed25519
cat id_ed25519.pub (copy)
go to github>settings>ssh and GPG keys > new ssh key > paste the key

ssh -T git@github.com


git branch -M main
git remote add origin git@github.com:prabh8331/Rust-lang.git
git push -u origin main

## Installing the rust
https://www.rust-lang.org/
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

rustc --version
cargo --verson

to update the rust
rustup update

to install all the depencenty in ubuntu
sudo apt update
sudo apt install build-essential


### installing the vscode extensions for rust
rust-analyzer
Rust Syntax
Dependi

