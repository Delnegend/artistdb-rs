rm -f artist-encoder
cargo build -p artist-encoder --release
mv target/release/artist-encoder artist-encoder
sudo chmod +x artist-encoder