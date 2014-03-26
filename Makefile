RUSTC ?= rustc
RUSTFLAGS ?= -O -L../rust-http/build -L../rust-postgres/build -L../rust-postgres/submodules/rust-phf/build -L../rust-postgres/submodules/rust-openssl/build 

evreserve:
	$(RUSTC) $(RUSTFLAGS) -o build/evreserve src/main.rs

clean:
	rm -rf build/*

all: evreserve
