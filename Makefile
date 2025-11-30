.PHONY: help wasm wasm-dev wasm-release clean install-tools check-wasm-pack info serve

# Default target
help:
	@echo "Available targets:"
	@echo "  make wasm          - Build optimized WASM for production (equivalent to wasm-release)"
	@echo "  make wasm-dev      - Build WASM for development (faster, larger)"
	@echo "  make wasm-release  - Build optimized WASM for production with all optimizations"
	@echo "  make serve         - Start local server with proper WASM MIME types"
	@echo "  make clean         - Remove build artifacts"
	@echo "  make install-tools - Install required tools (wasm-pack)"
	@echo "  make info          - Show WASM output information"
	@echo ""
	@echo "The generated_tables.bin is embedded in the WASM binary."

# Build production-optimized WASM (default target for 'make wasm')
wasm: wasm-release

# Development build (faster, larger file size)
wasm-dev: check-wasm-pack
	@echo "Building WASM for development..."
	wasm-pack build --target web --dev --scope hungarofit --out-dir web
	@echo ""
	@echo "Development build complete!"
	@$(MAKE) info

# Production build with all optimizations
wasm-release: check-wasm-pack
	@echo "Building WASM with release optimizations..."
	wasm-pack build --target bundler --release --scope hungarofit --out-dir pkg
	@echo ""
	@echo "Production build complete!"
	@$(MAKE) info

# Check if wasm-pack is installed
check-wasm-pack:
	@if ! command -v wasm-pack >/dev/null 2>&1; then \
		echo "Error: wasm-pack not found."; \
		echo "Run 'make install-tools' to install it, or install manually:"; \
		echo "curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh"; \
		exit 1; \
	fi

# Install required tools
install-tools:
	@echo "Installing wasm-pack..."
	@if ! command -v wasm-pack >/dev/null 2>&1; then \
		curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh; \
	else \
		echo "wasm-pack is already installed"; \
	fi

# Display information about the build output
info:
	@if [ -f pkg/evaluator_bg.wasm ]; then \
		echo ""; \
		echo "==============================================="; \
		echo "WASM Build Output:"; \
		echo "==============================================="; \
		ls -lh pkg/*.wasm 2>/dev/null || true; \
		echo ""; \
		if [ -f generated_tables.bin ]; then \
			echo "Tables binary size:"; \
			ls -lh generated_tables.bin; \
		fi; \
		echo ""; \
		echo "The generated_tables.bin is embedded in the WASM binary."; \
		echo "Total package size includes all tables data."; \
		echo "==============================================="; \
	else \
		echo "No WASM output found. Run 'make wasm' first."; \
	fi

# Start local development server with proper WASM MIME types
serve:
	@echo "Starting development server on http://localhost:8000"
	@echo "Open http://localhost:8000/wasm-example.html in your browser"
	@echo ""
	@echo "Note: This server properly serves .wasm files with 'application/wasm' MIME type"
	@echo "Press Ctrl+C to stop the server"
	@echo ""
	@cd web && npx http-server --cors -c-1

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	rm -rf pkg/
	rm -f generated_tables.bin
	cargo clean
	@echo "Clean complete!"
