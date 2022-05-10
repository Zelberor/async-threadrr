Important:
For devices with low RAM available for the browser (e.g. iOS devices) the max-memory of the wasm needs to be adjusted.
For iOS devices set the environment variable
WASM_BINDGEN_THREADS_MAX_MEMORY=8192
at compile time for 512MB of maximum available memory otherwise it will crash with an out of memory error.

Default seems to be 16384 (1GB).

For (android) devices with more RAM or desktops 65536 (4GB) can be used
