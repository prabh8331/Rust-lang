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

Links to Useful Books on Rust
List of Useful Resources for Learning Rust





Resources Covering Basic Concepts of the Language

1. Rust Programming Language

Online Link: https://doc.rust-lang.org/book/

Description: Official rust programming language book. Recommended book for those who are new to rust.

________________________________________________________________________

2. Rust by Example

Online Link: https://doc.rust-lang.org/rust-by-example/print.html

Description: A recommended book for those who want to quickly learn through examples with lesser focus on theory

________________________________________________________________________

3. Rust Cookbook

Online Link: https://rust-lang-nursery.github.io/rust-cookbook/print.html

Description: Useful Examples for accomplishing common programming tasks.

________________________________________________________________________

4. Easy Rust

Online Link: https://dhghomon.github.io/easy_rust/

Description: A text book for those who are not native speakers and want to learn with simple and easy to follow English.

________________________________________________________________________

5. Rust Reference

Online Link: https://doc.rust-lang.org/reference/introduction.html

Description: A good reference book for the language

_______________________________________________________________________

6. Rustlings (online resource only)

Online Link: https://github.com/rust-lang/rustlings

Description: Small exercises to get you used to reading and writing Rust code.

________________________________________________________________________

7. Rust CheatSheet

Online Link: https://zerotomastery.io/cheatsheets/rust-cheat-sheet/

Description: Quick reference guide and handy reference for programming in rust.

________________________________________________________________________

Resources Covering Advance Concepts of the Language

1. Comprehensive Rust

Online Link: https://google.github.io/comprehensive-rust/

Description: A recommended book for those who are coming from C++/JAVA background and want to develop for Andriod

_______________________________________________________________________

2. The Rust Performance Book

Online Link: https://nnethercote.github.io/perf-book/introduction.html

Description: The book contains techniques that can improve the performance, speed and memory usage

________________________________________________________________________

3. The Rustonomicon

Online Link: https://doc.rust-lang.org/nomicon/intro.html

Description: It contains many examples and codes that explains the details when writing unsafe Rust programs.

________________________________________________________________________



4. Learn Rust with Entirely Too Many Linked Lists

Online Link: https://rust-unofficial.github.io/too-many-lists/

Description: Basics of linked lists and different types of smart pointers in Rust.

________________________________________________________________________

5. Rust Programming Tips

Online Link: https://github.com/ferrous-systems/elements-of-rust/blob/master/README.md

Description: Useful programming examples and tips for effective rust programming.

________________________________________________________________________

Documentation of the Language

1. Standard Library

Online Link: https://doc.rust-lang.org/std/index.html

Description: Comprehensive guide to the Rust standard library APIs.

________________________________________________________________________

2. Edition Guide

Online Link: https://doc.rust-lang.org/edition-guide/index.html

Description: For understanding the Rust's way of introducing changes into the language

________________________________________________________________________

3. Cargo Book

Online Link: https://doc.rust-lang.org/cargo/index.html

Description: For understanding the package manager of Rust.

________________________________________________________________________

4. Compiler Error Index

Online Link: https://doc.rust-lang.org/error_codes/error-index.html

Description: Listing of all the errors emitted by the compiler

________________________________________________________________________

5. Rustdoc Book

Online Link: https://doc.rust-lang.org/rustdoc/index.html

Description: Understanding the generation of documentation for your code






Rust Reference Material/Cheat Sheets list

Extensive Level Cheat Sheets

1. The Rust Cheat Sheet by cheats.rs

Online Link: https://cheats.rs/

Description: An extensive cheat sheet touching almost all important aspects of the languages.

________________________________________________________________________

2. Rust in a Nutshell

Online Link: https://github.com/donbright/rust-lang-cheat-sheet/blob/master/README.md

Description: A detailed overview of the syntax covering both beginner and advance level topics.

________________________________________________________________________

3. Rust Cheat Sheet by Zero to Mastery

Online Link: https://zerotomastery.io/cheatsheets/rust-cheat-sheet/

Description: Another detailed and extensive cheat sheet.

________________________________________________________________________

Moderate level Cheat Sheet

4. LGR Rust Cheat

Online Link: https://docs.google.com/document/d/1kQidzAlbqapu-WZTuw4Djik0uTqMZYyiMXTM9F21Dz4/edit

Description: Intermediate level cheat sheet.

________________________________________________________________________

Compact Cheat Sheets

5. Rust Cheat Sheet by QuickRef.Me (Online Only)

Online Link: https://quickref.me/rust.html

Description: Brief and a quick reference.

________________________________________________________________________

6. Rust Cheat Sheet (One pager)

Online Link: https://phaiax.github.io/rust-cheatsheet/

Description: A one pager cheat sheet.
