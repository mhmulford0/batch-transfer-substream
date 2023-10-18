ENDPOINT ?= sepolia.eth.streamingfast.io:443
START_BLOCK ?= 4333100
STOP_BLOCK ?= 4333139

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: run
run: build
	substreams run -e $(ENDPOINT) substreams.yaml map_transfers -s $(START_BLOCK) -t $(STOP_BLOCK) -o jsonl

.PHONY: gui
gui: build
	substreams gui -e $(ENDPOINT) substreams.yaml map_transfers -s $(START_BLOCK) -t $(STOP_BLOCK)

.PHONY: protogen
protogen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"

.PHONY: package
package:
	rm -f *.spkg
	substreams pack ./substreams.yaml
