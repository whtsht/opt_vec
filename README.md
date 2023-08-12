# opt_vec

[![Crate](https://img.shields.io/crates/v/opt_vec.svg)](https://crates.io/crates/opt_vec)

A contiguous growable array type with heap-allocated contents
with fast deletion process.

This is a wrapper for [`Vec<Option<T>>`](https://doc.rust-lang.org/std/vec/struct.Vec.html)

## Use an OptVec when:

- You want fast random access and deletion,
  but don't want to use expensive structures like HashMap.
- You want to guarantee that the same index
  keeps the same value even if another element is removed.

## Getting Started

Cargo.toml

```text
[dependencies]
opt_vec = "*"
```

and then

```
use opt_vec::OptVec;

let mut v = OptVec::new();
v.push(1);
```

## Support `no_std`

Cargo.toml

```text
[dependencies.opt_vec]
version = "*"
default-features = false
features = ["alloc"]
```
