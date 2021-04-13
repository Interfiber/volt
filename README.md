# Volt
The Minecraft Forge Plugin Manager

## ℹ️ About
Everything you want to know is [here](https://interfiber.github.io/volt), the rest of this page is development docs.

## 👷 Building volt
Volt requires [↗️ Rust](https://rust-lang.org) to run, it also requires ```rustc 1.52.0-beta.3``` to run. After you have that clone this repo with git and run
```bash
cargo build --release
```
inside the cloned folder. All code is located under ```src```

## 📦 Packaging volt for Mac
To package volt for mac simply clone this repo and run
```bash
make pkg_mac
```

## 📦 Packaging for Linux
Volt on Linux is not avalible at the moment.