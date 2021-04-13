echo "Volt Build script"
echo "checking for rust..."
if ! test -f $(which rustc); then
    echo "rust not detected."
fi
if ! test -f $(which cargo); then
    echo "cargo not detected."
fi
echo "building..."
cargo build --release
mv ./target/debug/volt ./volt-$(uname)_$(uname -m)
echo "output file: ./volt-$(uname)_$(uname -m)"