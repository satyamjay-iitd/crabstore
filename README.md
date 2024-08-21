# Crabstore

Crabstore is(will be) an in memory distributed object store.

## Instructions

#### Build and Run Server

```sh
RUST_LOG=debug cargo run -- -s sock_path -m 1123
```

#### Install client

```sh
cd crates/crabstore-client
# create virtual environment
python3 -m venv venv
# switch to it
source venv/bin/activate
# install maturin
pip install maturin
# build
maturin develop
# run test -- you will need numpy (pip install numpy)
python examples/test_python_client.py  ../../sock_path
```
Make sure the socket path is correct in the client.


## TODOs
1. Figure out how to return mutable pointer to python from rust.
Efforts:-
  1. `pyo3::PyBytes` returns the bytes, but it is immutable.
  2. `pyo3::PyByteArray` returns the mutable bytes but it copies the data from the ptr (which defeats the whole fking point doesn't it?).
  3. [memoryview](https://docs.python.org/3/c-api/memoryview.html#memoryview-objects) looks promisiong, especially `PyMemoryView_FromMemory`.
   But highlevel API is not there in pyo3. Might need to implement it.
1. Implement the actual logic for seal:
  1. Client should pass the fd to server through UDS
  2. Server should maintain record of oids received from client
