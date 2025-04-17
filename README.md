# Build the Rust code for WebAssembly
cargo build --target wasm32-unknown-unknown --release

# Generate JS bindings using wasm-bindgen
wasm-bindgen target/wasm32-unknown-unknown/release/img_hash_wasm.wasm \
  --out-dir ./pkg \
  --target web


```js
import { init, bindImageCompare } from 'img-hash-wasm';

await init();

bindImageCompare({
  file1Input: document.getElementById('file1'),
  file2Input: document.getElementById('file2'),
  button: document.getElementById('compare-btn'),
  onResult: (diff) => {
    console.log('Difference score:', diff);
  },
});