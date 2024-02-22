rm -f artist-2-bincode
cd src-rust/artist-2-bincode-rs
cargo build --release
cd ../..
mv target/release/artist-2-bincode artist-2-bincode
sudo chmod +x artist-2-bincode