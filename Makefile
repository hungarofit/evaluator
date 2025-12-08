
.PHONY: build
build:
	cargo build

.PHONY: wasm-bundler
wasm-bundler:
	mkdir -p pkg/bundler
	wasm-pack build --release --target bundler --scope hungarofit --out-dir pkg/bundler

.PHONY: wasm-web
wasm-web:
	mkdir -p pkg/web
	wasm-pack build --release --target web --scope hungarofit --out-dir pkg/web
	
.PHONY: wasm
wasm: wasm-bundler wasm-web

.PHONY: info
info:
	ls -lah pkg/**/*.wasm

.PHONY: all
all: build wasm info

.PHONY: serve
serve:
	node dev/web.js

.PHONY: clean
clean:
	rm -rf pkg/
	cargo clean
