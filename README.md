# Volt
The Minecraft Forge Plugin Manager

## âšī¸ About
Everything you want to know is [here](https://interfiber.github.io/volt), the rest of this page is development docs.

## đˇ Building volt
Volt requires [âī¸ Rust](https://rust-lang.org) to run, it also requires ```rustc 1.52.0-beta.3``` to run. After you have that clone this repo with git and run
```bash
cargo build --release
```
inside the cloned folder. All code is located under ```src```

## đĻ Packaging volt for Mac
To package volt for mac simply clone this repo and run
```bash
cargo build --release
make pkg_mac
```

## đĻ Packaging for Linux
To package volt for linux you need to have a linux machine.
Then clone this repo and run the following command in the cloned folder
```bash
cargo build --release
make pkg_linux
```
This will generate a tarball which will contain a example config file, and the volt binary
