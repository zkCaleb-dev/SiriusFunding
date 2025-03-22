# Ejecuta build por defecto
default: build

# Alias para test
all: test

# Test
test: build
	cargo test

# Build del contrato
build:
	stellar contract build

# Deploy del contrato
deploy: build
	stellar contract deploy \
		--wasm ../../target/wasm32-unknown-unknown/release/sirius.wasm \
		--source zkCaleb \
		--network testnet \
		--alias sirius

# Formatea el código
fmt:
	cargo fmt --all

# Limpia el proyecto
clean:
	cargo clean
