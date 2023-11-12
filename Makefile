all:
	cargo build
run:
	cat addresses.txt | RUST_LOG=trace ./target/debug/dianadb_app
