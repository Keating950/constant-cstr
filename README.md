# constant-cstr
[<img alt="crates.io" src="https://img.shields.io/crates/v/constant-cstr">](https://crates.io/crates/constant-cstr)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/constant-cstr">](https://docs.rs/constant-cstr)
### Create const instances of CStr at compile time

`constant-cstr` exists to enable the safe creation of
[`CStr`](https://doc.rust-lang.org/stable/core/ffi/struct.CStr.html) instances
at compile time, enabling safer and more efficient FFI. It leverages the
recently added expansions to const functions to verify the absence of any
interior null bytes.
