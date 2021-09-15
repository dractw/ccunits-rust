# ccunits-rust
Rust implementation of [ccunits](https://github.com/dractw/ccunits), wasm-ready

# Requirements
- wasm-pack

# build 
- Web: `wasm-pack build --target web`
- Bundler: `wasm-pack build --target bundler`
- Node: `wasm-pack build --target node`

# wasm usage

```js
import * as ccunits from 'ccunits-rust'

const mins = ccunits.to_minimal('134.677452125', 18) // "134677452125000000000"
const currency = ccunits.from_minimal('134677452125000000000', 18) // '134.677452125'
```