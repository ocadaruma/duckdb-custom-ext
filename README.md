An example of custom duckdb extension which runs on duckdb-wasm.

## How to run

- Prerequisites
  * emsdk
  * rust toolchain
      - wasm32-unknown-emscripten
  * cmake

Build:
```bash
# NOTE: You should activate emsdk before run

% cd /path/to/duckdb-custom-ext
% git submodule update --init --recursive
% cargo build --release --target wasm32-unknown-emscripten
% make -C duckdb-wasm \
  DUCKDB_SKIP_BUILD_EH=1 \
  DUCKDB_SKIP_BUILD_COI=1 \
  CUSTOM_EXTENSION_DIRS=$PWD \
  wasm wasmpack shell docs app_start
```

Go localhost:9002 and open console:

<img width="507" alt="image" src="https://user-images.githubusercontent.com/6070684/265238826-7930439f-0871-4fc9-8f05-1b8c025b2c2e.png">

You should see `hello from extension!` on the console!
