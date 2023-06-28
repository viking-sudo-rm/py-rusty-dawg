# (Py) Rusty DAWG

A Python wrapper for Rusty DAWG, providing seamless access to fast and memory-efficient DAWG data structures implemented in Rust.

## Building

First update `rusty-dawg` to the most recent version from the GitHub repo:

```
cargo update -p rusty-dawg
```

Then build via Maturin:

```
maturin build
```

Finally, install the generated wheel via pip in your Python installation:

```
pip install target/wheels/*.whl
```