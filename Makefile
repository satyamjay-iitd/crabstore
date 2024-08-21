run_server:
	RUST_LOG=debug cargo run -- -s /tmp/sock_path -m 1123 -d 123123

run_client:
	cd crates/crabstore-client; cargo run --example demo
