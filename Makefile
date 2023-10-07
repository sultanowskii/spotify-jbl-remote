SRCS=$(wildcard src/*.rs src/*/*.rs)
BIN=target/release/spotify-jbl-remote

.PHONY: all
all: build

.PHONY: build
build: $(SRCS) 
	cargo build --release

.PHONY: debug-build
debug-build: $(SRCS)
	cargo build

.PHONY: clean
clean:
	cargo clean

.PHONY: install
install:
	install -m 755 $(BIN) /usr/bin/
	cp scripts/51-jbl.rules /etc/udev/rules.d/
