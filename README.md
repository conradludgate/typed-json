# Typed JSON &emsp; [![Latest Version]][crates.io]

[Latest Version]: https://img.shields.io/crates/v/typed-json.svg
[crates.io]: https://crates.io/crates/typed-json

Typed JSON provides a `json!` macro to build an `impl serde::Serialize`
type with very natural JSON syntax.

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

fn random_phone() -> String {
    "0".to_owned()
}

// The type of `john` is `impl serde::Serialize`
let john = typed_json::json!({
    "name": full_name,
    "age": age_last_year + 1,
    "phones": [
        format!("+44 {}", random_phone())
    ]
});
```

# Comparison to `serde_json`

This crate provides a typed version of [`serde_json::json!()`](https://docs.rs/serde_json/latest/serde_json/macro.json.html).
What does that mean? It means it performs 0 allocations and it creates a custom type for the JSON object you are representing.
For one-off JSON documents, this ends up being considerably faster to encode.
This is 100% compatible with [`serde_json::json!()`](https://docs.rs/serde_json/latest/serde_json/macro.json.html)
syntax as of `serde_json = "1.0.108"`.

## Benchmark

The following benchmarks indicate serializing a complex deeply-nested JSON document to a `String`.

> Note: the `typed_json_core` benchmark uses [`serde-json-core`](https://docs.rs/serde-json-core/latest/serde_json_core/index.html) to encode to a `heapless::String`.

```sh
Timer precision: 41 ns
serialize_string    fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ serde_json       739 ns        │ 6.437 µs      │ 780.8 ns      │ 824.2 ns      │ 100000  │ 400000
├─ typed_json       172.7 ns      │ 852.4 ns      │ 176.6 ns      │ 178.5 ns      │ 100000  │ 3200000
╰─ typed_json_core  209.2 ns      │ 1.249 µs      │ 219.6 ns      │ 222 ns        │ 100000  │ 3200000
```

> Note: The benchmarks use [`serde_json::to_string`](https://docs.rs/serde_json/latest/serde_json/fn.to_string.html)
> as it's significantly faster than the `ToString`/`Display` implementation, both for `serde_json::json` and `typed_json::json`

# No-std support

It is possible to use `typed_json` with only `core`. Disable the default "std"
feature:

```toml
[dependencies]
typed_json = { version = "0.1", default-features = false }
```

To encode the `Serialize` type to JSON:

you will either need [`serde_json`](https://docs.rs/serde_json/latest/serde_json/index.html) with the `alloc` feature

```toml
[dependencies]
serde_json = { version = "1.0", default-features = false, features = ["alloc"] }
```

or [`serde-json-core`](https://docs.rs/serde-json-core/latest/serde_json_core/index.html) with no dependency on `alloc`

```toml
[dependencies]
serde-json-core = "0.5.1"
```

# How it works

> Note: all of this is implementation detail and **none of this is stable API**

```rust,ignore
let data = typed_json::json!({
    "codes": [value1, value2],
    "message": value3
})
```

Expands into

```rust,ignore
typed_json::__private::Map(
    typed_json::__private::HList {
      first: Some(typed_json::__private::KV::Pair(
          typed_json::__private::Expr("codes"),
          typed_json::__private::Array(
              typed_json::__private::HList {
                  first: Some(typed_json::__private::Expr(value1)),
                  second: Some(typed_json::__private::Expr(value2)),
              },
          ),
      )),
      second: Some(typed_json::__private::KV::Pair(
          typed_json::__private::Expr("message"),
          typed_json::__private::Expr(value3),
      )),
  }
)
```

# Compile time benchmarks

There's no such thing as a true zero-cost abstraction. However, it seems that sometimes
`typed-json` compiles faster than `serde_json`, and sometimes the opposite is true.

I measured the compile times using the large service JSON from <https://kubernetesjsonschema.dev/>.

<details>
<summary>Benchmark details</summary>

## Many small documents

In this test, I have split the above JSON file into 31 reasonably-sized documents

### Debug

```sh
$ hyperfine \
    --command-name "typed_json" \
    "pushd tests/crates/stress3 && touch src/main.rs && cargo build" \
    --command-name "serde_json" \
    "pushd tests/crates/stress4 && touch src/main.rs && cargo build"

Benchmark 1: typed_json
  Time (mean ± σ):     143.4 ms ±   5.8 ms    [User: 129.7 ms, System: 74.8 ms]
  Range (min … max):   138.0 ms … 161.6 ms    20 runs
 
Benchmark 2: serde_json
  Time (mean ± σ):     156.3 ms ±   5.1 ms    [User: 131.8 ms, System: 94.9 ms]
  Range (min … max):   150.4 ms … 173.5 ms    18 runs
 
Summary
  typed_json ran
    1.09 ± 0.06 times faster than serde_json
```

### Release

```sh
$ hyperfine \
    --command-name "typed_json" \
    "pushd tests/crates/stress3 && touch src/main.rs && cargo build --release" \
    --command-name "serde_json" \
    "pushd tests/crates/stress4 && touch src/main.rs && cargo build --release"

Benchmark 1: typed_json
  Time (mean ± σ):     581.5 ms ±   8.7 ms    [User: 908.2 ms, System: 68.9 ms]
  Range (min … max):   573.3 ms … 602.8 ms    10 runs
 
Benchmark 2: serde_json
  Time (mean ± σ):      1.079 s ±  0.018 s    [User: 1.268 s, System: 0.082 s]
  Range (min … max):    1.059 s …  1.123 s    10 runs
 
Summary
  typed_json ran
    1.86 ± 0.04 times faster than serde_json
```

## One-off large document

In this test, I have included the single JSON file in verbatim.
I don't think this is a realistic use case but still interesting

### Debug

```sh
$ hyperfine \
    --command-name "typed_json" \
    "pushd tests/crates/stress1 && touch src/main.rs && cargo build" \
    --command-name "serde_json" \
    "pushd tests/crates/stress2 && touch src/main.rs && cargo build"

Benchmark 1: typed_json
  Time (mean ± σ):     135.8 ms ±   2.9 ms    [User: 132.8 ms, System: 73.7 ms]
  Range (min … max):   132.5 ms … 143.8 ms    22 runs

Benchmark 2: serde_json
  Time (mean ± σ):     151.8 ms ±   6.2 ms    [User: 133.8 ms, System: 98.1 ms]
  Range (min … max):   144.3 ms … 171.6 ms    20 runs

Summary
  typed_json ran
    1.12 ± 0.05 times faster than serde_json
```

### Release

```sh
$ hyperfine \
    --command-name "typed_json" \
    "pushd tests/crates/stress1 && touch src/main.rs && cargo build --release" \
    --command-name "serde_json" \
    "pushd tests/crates/stress2 && touch src/main.rs && cargo build --release"

Benchmark 1: typed_json
  Time (mean ± σ):      1.881 s ±  0.034 s    [User: 2.765 s, System: 0.094 s]
  Range (min … max):    1.810 s …  1.933 s    10 runs

Benchmark 2: serde_json
  Time (mean ± σ):     931.1 ms ±  14.4 ms    [User: 1132.1 ms, System: 70.3 ms]
  Range (min … max):   903.4 ms … 943.2 ms    10 runs

Summary
  serde_json ran
    2.02 ± 0.05 times faster than typed_json
```

</details>

## Conclusion

I don't think I can conclusively say that typed-json introduces a compile-time regression in standard use.
At the extremes, it likely will need to compile many more types but in standard use, it can re-use a lot of prior compilations.
