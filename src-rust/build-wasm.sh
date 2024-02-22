rm -rf src/composables/bincode-2-artist-wasm

cd src-rust/bincode-2-artist-wasm
wasm-pack build --release
cd ../..

mv src-rust/bincode-2-artist-wasm/pkg src/composables/bincode-2-artist-wasm
rm -rf .nuxt
rm -f src/composables/bincode-2-artist-wasm/.gitignore
pnpm i