use std::env;

fn main() {
    // if WASM_BINDGEN_THREADS_MAX_MEMORY is set => set the correct linker args
    println!("cargo:rerun-if-env-changed=WASM_BINDGEN_THREADS_MAX_MEMORY");
    if let Ok(mem_str) = env::var("WASM_BINDGEN_THREADS_MAX_MEMORY") {
        let max_memory: i32 = mem_str
            .parse()
            .expect("WASM_BINDGEN_THREADS_MAX_MEMORY must be an integer");
        println!(
            "cargo:rustc-link-arg=--max-memory={}",
            max_memory * 64 * 1024
        );
    }
}
