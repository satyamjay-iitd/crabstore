run_server:
	RUST_LOG=debug cargo run -- -s /tmp/sock_path -m 1123

run_client:
	cd crates/crabstore-client; RUST_LOG=debug cargo run --example demo

run_python_client:
	cd crates/crabstore-python-client; . venv/bin/activate; maturin develop && python examples/test_python_client.py /tmp/sock_path
