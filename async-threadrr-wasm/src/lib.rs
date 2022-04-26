#[cfg(not(target_feature = "atomics"))]
compile_error!("The compiler features `atomics` and `bulk-memory` need to be enabled for async-threadrr-wasm to work. (Rust nightly required)");
