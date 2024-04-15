cargo build --release --target wasm32-unknown-unknown
cargo install wasm-bindgen-cli
wasm-bindgen --no-typescript --target web \
	--out-dir ./web/ \
	--out-name "pestilence" \
	./target/wasm32-unknown-unknown/release/pestilence.wasm

cd web
cp -r ../assets .
zip ../pestilence.zip assets/**/* index.html pestilence.js pestilence_bg.wasm
rm -r ./assets
