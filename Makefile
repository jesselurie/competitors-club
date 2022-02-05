.PHONY: init
init:
	./scripts/init.sh

.PHONY: check
check:
	SKIP_WASM_BUILD=1 cargo check --release

.PHONY: test
test:
	SKIP_WASM_BUILD=1 cargo test --release --all

.PHONY: run
run:
	cargo run --release -- --dev --tmp --offchain-worker=Always

.PHONY: build
build:
	cargo build --manifest-path node/Cargo.toml --release --features runtime-benchmarks

.PHONY: bench-list
bench-list:
	./target/release/competitors-club-node benchmark --chain dev --pallet "*" --extrinsic "*" --repeat 0

.PHONY: benchmark-output
benchmark-output:
	cd runtime/src/weights && ../../../target/release/competitors-club-node benchmark --chain dev --execution=wasm --wasm-execution=compiled --pallet pallet_sovereign_assets --extrinsic '*' --steps 50 --repeat 20 --output

.PHONY: benchmark-traits
benchmark-traits:
	cd runtime/src/weights && ../../../target/release/competitors-club-node benchmark --chain dev --execution=wasm --wasm-execution=compiled --pallet pallet_sovereign_assets --extrinsic '*' --steps 50 --repeat 20 --weight-trait --output

.PHONY: benchmark-help
benchmark-help:
	./target/release/competitors-club-node benchmark --help

.PHONY: test-benchmarks
test-benchmarks:
	cargo test --manifest-path pallets/sovereign-assets/Cargo.toml --features runtime-benchmarks -- --nocapture

.PHONY: chain-spec
chain-spec: 
	./target/release/competitors-club-node build-spec --disable-default-bootnode --chain dev > mainnetSpec.json

.PHONY: chain-spec-raw
chain-spec-raw:
	./target/release/competitors-club-node build-spec --chain=mainnetSpec.json --raw --disable-default-bootnode > mainnetSpecRaw.json

.PHONY: single-node
single-node:
	./target/release/competitors-club-node --base-path data/alice --chain=mainnetSpecRaw.json --alice --name alice --force-authoring

.PHONY:multi-node
multi-node:
	cargo run --base-path data/alice --chain=mainnetSpec.json --alice --name alice --validator
	cargo run --base-path data/bob --chain=mainnetSpec.json --bob --name bob --validator
	
.PHONY: purge
purge:
	./target/release/competitors-club-node purge-chain –chain=mainnet -d data/alice
	./target/release/competitors-club-node purge-chain –chain=mainnet -d data/bob

.PHONY: node0
node0:
	./target/release/competitors-club-node --chain=mainnetSpecRaw.json --base-path ./tmp/node0 --node-key=8db0e0bb7f6a82c659aa185d4f487a2fe738f27e8c18067a5dc955d89441c8d8 --port 30333 --ws-port 9944 --rpc-port 9933 --validator --offchain-worker=Always

.PHONY: node1
node1:
	./target/release/competitors-club-node --chain=mainnetSpecRaw.json --base-path ./tmp/node1  --bootnodes /ip4/0.0.0.0/tcp/30333/p2p/12D3KooWAGhwMEWRstpwtVmVkxFznbakjhEDNMTTZMKhiQhUoKEQ --node-key=5f881beb1c0af7858aa6cab672ace1cdc94ce14e14f1c9a51fa1d6c661af9b9e --port 30334 --ws-port 9945 --rpc-port 9934 --validator --offchain-worker=Always

.PHONY: node2
node2:
	./target/release/competitors-club-node --chain=mainnetSpecRaw.json --base-path ./tmp/node2  --bootnodes /ip4/0.0.0.0/tcp/30333/p2p/12D3KooWAGhwMEWRstpwtVmVkxFznbakjhEDNMTTZMKhiQhUoKEQ --node-key=6182d4322d96435d64adf1416a94fba99a02e6e112d8669d784a334d817e2701 --port 30335 --ws-port 9946 --rpc-port 9935 --validator --offchain-worker=Always


build-full:
	WASM_TARGET_DIRECTORY="/Users/jkl/Documents/competitors-club-node/wasm" cargo build
