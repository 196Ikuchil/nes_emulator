build:
	mkdir -p wasm
	rm -rf target/wasm32-unknown-emscripten/release/deps/*.wasm
	rm -rf target/wasm32-unknown-emscripten/release/nes_emulator.js
	cargo rustc --release --bin nes_emulator \
	--target=wasm32-unknown-emscripten -- \
    -C opt-level=3 \
	-C link-args="-O3 -s NO_EXIT_RUNTIME=1 -s EXPORTED_FUNCTIONS=['_run'] -s EXTRA_EXPORTED_RUNTIME_METHODS=['cwrap']" \
	--verbose
	cp target/wasm32-unknown-emscripten/release/nes_emulator.js wasm/nes_emulator.js
	cp target/wasm32-unknown-emscripten/release/deps/*.wasm wasm/nes_emulator.wasm

clean:
	rm -rf target/wasm32-unknown-emscripten/release/deps/*.wasm
	rm -rf target/wasm32-unknown-emscripten/release/nes_emulator.js
	rm -rf target/release