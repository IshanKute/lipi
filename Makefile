.PHONY: install uninstall build clean

# Build release version
build:
	cargo build --release

# Install Lipi locally
install: build
	cargo install --path .

# Uninstall Lipi
uninstall:
	cargo uninstall lipi

# Clean build artifacts
clean:
	cargo clean
