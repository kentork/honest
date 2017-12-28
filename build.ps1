cargo +nightly build --target wasm32-unknown-unknown --release

if ($? -eq $true) {
  # cp ./target/wasm32-unknown-unknown/release/honest.wasm ./public/wasm/
  wasm-gc ./target/wasm32-unknown-unknown/release/honest.wasm ./public/wasm/honest.wasm
}