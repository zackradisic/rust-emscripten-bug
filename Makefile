.PHONY:
wasm:
	EMMAKEN_CFLAGS="-s ERROR_ON_UNDEFINED_SYMBOLS=0 --no-entry" cargo build -Zextra-link-arg --release --target=wasm32-unknown-emscripten