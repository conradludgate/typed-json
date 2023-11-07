# Typed JSON &emsp; [![Latest Version]][crates.io]

[Latest Version]: https://img.shields.io/crates/v/typed-json.svg
[crates.io]: https://crates.io/crates/typed-json

Typed JSON provides a `json!` macro to build an `impl serde::Serialize`
objects with very natural JSON syntax.

```rust
use typed_json::json;

// The type of `john` is `impl serde::Serialize`
let john = json!({
    "name": "John Doe",
    "age": 43,
    "phones": [
        "+44 1234567",
        "+44 2345678"
    ]
});

// Convert to a string of JSON and print it out
println!("{}", serde_json::to_string(&john).unwrap());
```

One neat thing about the `json!` macro is that variables and expressions can
be interpolated directly into the JSON value as you are building it. Serde
will check at compile time that the value you are interpolating is able to
be represented as JSON.

```rust
let full_name = "John Doe";
let age_last_year = 42;

// The type of `john` is `impl serde::Serialize`
let john = json!({
    "name": full_name,
    "age": age_last_year + 1,
    "phones": [
        format!("+44 {}", random_phone())
    ]
});
```

This is amazingly convenient, but we have the problem we had before with
`Value`: the IDE and Rust compiler cannot help us if we get it wrong. Serde
JSON provides a better way of serializing strongly-typed data structures
into JSON text.

# No-std support

It is possible to use typed_json with only `core`. Disable the default "std"
feature:

```toml
[dependencies]
typed_json = { version = "0.1", default-features = false }
```

To encode the `Serialize`` type to JSON:

you will either need [`serde_json`](https://docs.rs/serde_json/latest/serde_json/index.html) with the `alloc`` feature
```toml
[dependencies]
serde_json = { version = "1.0", default-features = false, features = ["alloc"] }
```

or [`serde-json-core`](https://docs.rs/serde-json-core/latest/serde_json_core/index.html) with no dependency on `alloc`
```toml
[dependencies]
serde-json-core = "0.5.1"
```

# Comparison to `serde_json`

This crate provides a typed version of `serde_json::json!()`. What does that mean? It means it performs 0 allocations and it creates
a custom type for the JSON object you are representing. For one-off JSON documents, this ends up being considerably faster to encode.
This is 100% compatible with `serde_json::json!` syntax as of `serde_json = "1.0.108"`.

## Benchmark

The following benchmarks indicate serializing a complex deeply-nested JSON document to a `String`.
the `typed_json_core` benchmark uses `serde-json-core` to encode to a `heapless::String`.

```
Timer precision: 41 ns
serialize_string    fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ serde_json       707 ns        │ 36.62 µs      │ 791 ns        │ 1.096 µs      │ 10000   │ 10000
├─ typed_json       154 ns        │ 844.1 ns      │ 163.1 ns      │ 163.5 ns      │ 10000   │ 320000
╰─ typed_json_core  215.2 ns      │ 742.5 ns      │ 229.5 ns      │ 229.7 ns      │ 10000   │ 320000
```
