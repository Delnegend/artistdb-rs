rm -rf src/composables/bridge

cd src-rust/bridge
wasm-pack build --release
cd ../..

mv src-rust/bridge/pkg src/composables/bridge
rm -rf .nuxt
rm -f src/composables/bridge/.gitignore
bun i