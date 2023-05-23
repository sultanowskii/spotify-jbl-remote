build:
	cargo build --release

install:
	install -m 755 target/release/spotify-jbl-remote /usr/bin/
	cp scripts/51-jbl.rules /etc/udev/rules.d/
