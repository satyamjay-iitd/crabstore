# Crabstore

Crabstore is(will be) an in memory distributed object store.

## Instructions
#### Run Server
> RUST_LOG=debug cargo run -- -s sock_path -m 1123 -d 123123

#### Install client
```python
pip install maturin
cd crates/crabstore-client
maturin develop
python crates/crabstore-client/examples/test_python_client.py
```
Make sure the socket path is correct in the client.


## TODOs
1. Figure out how to return mutable pointer to python from rust.
Efforts:-
  1. `pyo3::PyBytes` returns the bytes, but it is immutable.
  2. `pyo3::PyByteArray` returns the mutable bytes but it copies the data from the ptr (which defeats the whole fking point doesn't it?).
  3. [memoryview](https://docs.python.org/3/c-api/memoryview.html#memoryview-objects) looks promisiong, especially `PyMemoryView_FromMemory`.
   But highlevel API is not there in pyo3. Might need to implement it.
