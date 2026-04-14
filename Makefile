PACKAGE  := liquidity_party
VERSION  := v0.1.0
SPKG     := $(PACKAGE)-$(VERSION).spkg

# Ethereum mainnet endpoint — override with ENDPOINT=... on the command line
ENDPOINT  ?= mainnet.eth.streamingfast.io:443
START_BLOCK ?= 24537169
STOP_BLOCK  ?= 24537381

.PHONY: build pack publish run gui clean

## Build the WASM binary (release profile, wasm32 target)
build:
	cargo build --target wasm32-unknown-unknown --release

## Bundle the WASM + manifest + protos into a .spkg file
pack: build
	substreams pack substreams.yaml -o $(SPKG)

## Publish the .spkg to the Substreams.dev registry
## Requires SUBSTREAMS_API_TOKEN to be set in your environment
publish: pack
	substreams registry publish $(SPKG)

## Stream map_pool_metrics against mainnet and print JSON output
## Usage: make run  or  make run START_BLOCK=24535806 STOP_BLOCK=+50
run: pack
	substreams run -e $(ENDPOINT) $(SPKG) map_pool_metrics \
		-s $(START_BLOCK) \
		-t $(STOP_BLOCK) \
		-o json

## Launch the interactive Substreams GUI for exploration
## Usage: make gui  or  make gui START_BLOCK=24535806 STOP_BLOCK=+1000
gui: pack
	substreams gui -e $(ENDPOINT) $(SPKG) map_pool_metrics \
		-s $(START_BLOCK) \
		-t $(STOP_BLOCK)

## Remove build artifacts and the packaged .spkg
clean:
	cargo clean
	rm -f $(SPKG)
