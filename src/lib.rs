//! # Typed JSON
//!
//! Typed JSON provides a [`json!` macro][crate::json] to build an [`impl serde::Serialize`](serde::Serialize)
//! type with very natural JSON syntax.
//!
//! ```
//! use typed_json::json;
//!
//! // The type of `john` is `impl serde::Serialize`
//! let john = json!({
//!     "name": "John Doe",
//!     "age": 43,
//!     "phones": [
//!         "+44 1234567",
//!         "+44 2345678"
//!     ]
//! });
//!
//! // Convert to a string of JSON and print it out
//! println!("{}", john.to_string());
//! ```
//!
//! One neat thing about the `json!` macro is that variables and expressions can
//! be interpolated directly into the JSON value as you are building it. Serde
//! will check at compile time that the value you are interpolating is able to
//! be represented as JSON.
//!
//! ```
//! # use typed_json::json;
//! #
//! # fn random_phone() -> u16 { 0 }
//! #
//! let full_name = "John Doe";
//! let age_last_year = 42;
//!
//! // The type of `john` is `impl serde::Serialize`
//! let john = json!({
//!     "name": full_name,
//!     "age": age_last_year + 1,
//!     "phones": [
//!         format!("+44 {}", random_phone())
//!     ]
//! });
//! ```
//!
//! # Comparison to `serde_json`
//!
//! This crate provides a typed version of [`serde_json::json!()`](https://docs.rs/serde_json/1.0.108/serde_json/macro.json.html). What does that mean? It means it performs 0 allocations and it creates
//! a custom type for the JSON object you are representing. For one-off JSON documents, this ends up being considerably faster to encode.
//! This is 100% compatible with `serde_json::json!` syntax as of `serde_json = "1.0.108"`.
//!
//! ## Benchmark
//!
//! The following benchmarks indicate serializing a complex deeply-nested JSON document to a `String`.
//! The `typed_json_core` benchmark uses [`serde-json-core`](https://docs.rs/serde-json-core/latest/serde_json_core/index.html)
//! to encode to a [`heapless::String`](https://docs.rs/heapless/0.7.16/heapless/index.html).
//!
//! ```text
//! Timer precision: 41 ns
//! serialize_string    fastest       │ slowest       │ median        │ mean          │ samples │ iters
//! ├─ serde_json       707 ns        │ 36.62 µs      │ 791 ns        │ 1.096 µs      │ 10000   │ 10000
//! ├─ typed_json       154 ns        │ 844.1 ns      │ 163.1 ns      │ 163.5 ns      │ 10000   │ 320000
//! ╰─ typed_json_core  215.2 ns      │ 742.5 ns      │ 229.5 ns      │ 229.7 ns      │ 10000   │ 320000
//! ```
//!
//! > Note: The benchmarks use `serde_json::to_string` as it's significantly faster than the `ToString`/`Display` implementation,
//! > both for `serde_json::json` and `typed_json::json`
//!
//! # No-std support
//!
//! It is possible to use `typed_json` with only `core`. Disable the default "std"
//! feature:
//!
//! ```toml
//! [dependencies]
//! typed_json = { version = "0.1", default-features = false }
//! ```
//!
//! To encode the `Serialize` type to JSON:
//!
//! you will either need [`serde_json`](https://docs.rs/serde_json/latest/serde_json/index.html) with the `alloc` feature
//! ```toml
//! [dependencies]
//! serde_json = { version = "1.0", default-features = false, features = ["alloc"] }
//! ```
//!
//! or [`serde-json-core`](https://docs.rs/serde-json-core/latest/serde_json_core/index.html) with no dependency on `alloc`
//! ```toml
//! [dependencies]
//! serde-json-core = "0.5.1"
//! ```
//!
//! # Compile time benchmarks
//!
//! There's no such thing as a true zero-cost abstraction.
//!
//! I measured the compile times using the large service JSON from <https://kubernetesjsonschema.dev/>.
//!
//! <details class="custom">
//!     <summary>Test details</summary>
//!
//! ## Many small documents
//!
//! In this test, I have split the above JSON file into 31 reasonably-sized documents
//!
//! ### Debug
//!
//! ```sh
//! $ hyperfine \
//!     --command-name "typed_json" \
//!     "pushd tests/crates/stress3 && touch src/main.rs && cargo build" \
//!     --command-name "serde_json" \
//!     "pushd tests/crates/stress4 && touch src/main.rs && cargo build"
//!
//! Benchmark 1: typed_json
//!   Time (mean ± σ):     132.2 ms ±   3.7 ms    [User: 127.9 ms, System: 73.4 ms]
//!   Range (min … max):   126.3 ms … 140.9 ms    22 runs
//!
//! Benchmark 2: serde_json
//!   Time (mean ± σ):     145.7 ms ±   3.6 ms    [User: 131.9 ms, System: 97.5 ms]
//!   Range (min … max):   139.6 ms … 153.2 ms    20 runs
//!
//! Summary
//!   typed_json ran
//!     1.10 ± 0.04 times faster than serde_json
//! ```
//!
//! ### Release
//!
//! ```sh
//! $ hyperfine \
//!     --command-name "typed_json" \
//!     "pushd tests/crates/stress3 && touch src/main.rs && cargo build --release" \
//!     --command-name "serde_json" \
//!     "pushd tests/crates/stress4 && touch src/main.rs && cargo build --release"
//!
//! Benchmark 1: typed_json
//!   Time (mean ± σ):     632.7 ms ±  10.2 ms    [User: 961.5 ms, System: 70.2 ms]
//!   Range (min … max):   610.5 ms … 647.2 ms    10 runs
//!
//! Benchmark 2: serde_json
//!   Time (mean ± σ):      1.001 s ±  0.020 s    [User: 1.188 s, System: 0.077 s]
//!   Range (min … max):    0.973 s …  1.029 s    10 runs
//!
//! Summary
//!   typed_json ran
//!     1.58 ± 0.04 times faster than serde_json
//! ```
//!
//! ## One off large document
//!
//! In this test, I have included the single JSON file in verbatim.
//! I don't think this is a realistic use case but still interesting
//!
//! ### Debug
//!
//! ```sh
//! $ hyperfine \
//!     --command-name "typed_json" \
//!     "pushd tests/crates/stress1 && touch src/main.rs && cargo build" \
//!     --command-name "serde_json" \
//!     "pushd tests/crates/stress2 && touch src/main.rs && cargo build"
//!
//! Benchmark 1: typed_json
//!   Time (mean ± σ):     137.7 ms ±   4.6 ms    [User: 134.5 ms, System: 74.9 ms]
//!   Range (min … max):   128.1 ms … 146.1 ms    21 runs
//!
//! Benchmark 2: serde_json
//!   Time (mean ± σ):     148.5 ms ±   6.8 ms    [User: 133.5 ms, System: 101.3 ms]
//!   Range (min … max):   139.3 ms … 164.6 ms    17 runs
//!
//! Summary
//!   typed_json ran
//!     1.08 ± 0.06 times faster than serde_json
//! ```
//!
//! ### Release
//!
//! ```sh
//! $ hyperfine \
//!     --command-name "typed_json" \
//!     "pushd tests/crates/stress1 && touch src/main.rs && cargo build --release" \
//!     --command-name "serde_json" \
//!     "pushd tests/crates/stress2 && touch src/main.rs && cargo build --release"
//!
//! Benchmark 1: typed_json
//!   Time (mean ± σ):      2.616 s ±  0.014 s    [User: 3.932 s, System: 0.118 s]
//!   Range (min … max):    2.588 s …  2.638 s    10 runs
//!
//! Benchmark 2: serde_json
//!   Time (mean ± σ):      1.281 s ±  0.014 s    [User: 1.554 s, System: 0.088 s]
//!   Range (min … max):    1.268 s …  1.305 s    10 runs
//!
//! Summary
//!   serde_json ran
//!     2.04 ± 0.02 times faster than typed_json
//! ```
//!
//! </details>
//!
//! ## Conclusion
//!
//! I don't think I can conclusively say that typed-json introduces a compile time regression in standard use.
//! At the extremes it likely will need to compile many more types but in standard use it can re-use a lot of prior compilations.

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(
    feature = "specialization",
    allow(incomplete_features),
    feature(specialization)
)]

#[macro_use]
mod macros;
mod expr_de;

#[cfg(feature = "std")]
mod fmt;

#[path = "zst_expr.rs"]
#[doc(hidden)]
/// Not part of the public API
pub mod ඞ;

/// A clone of [`serde::de::Deserializer`] to get around the orphan rule
pub trait Deserializer<'de>: Sized {
    /// Require the `Deserializer` to figure out how to drive the visitor based
    /// on what data type is in the input.
    fn deserialize_any2<V>(self, visitor: V) -> Result<V::Value, serde::de::value::Error>
    where
        V: serde::de::Visitor<'de>;
}

#[derive(Clone, Copy)]
#[doc(hidden)]
pub struct Expr<T>(pub T);

impl<S1: serde::ser::Serialize> serde::ser::Serialize for Expr<S1> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

#[derive(Clone, Copy)]
#[doc(hidden)]
pub struct Null;

impl<'de> crate::Deserializer<'de> for Null {
    fn deserialize_any2<V>(self, visitor: V) -> Result<V::Value, serde::de::value::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_none()
    }
}

impl<'de> serde::de::Deserializer<'de> for Null {
    type Error = serde::de::value::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_none()
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

impl serde::ser::Serialize for Null {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_none()
    }
}

#[doc(hidden)]
#[derive(Clone, Copy)]
pub enum KV<T, U> {
    Pair(T, U),
    V(U),
}

#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct KVList<T, U, V> {
    pub first: Option<KV<T, U>>,
    pub second: V,
}

impl<'de, T, U, V> KeyValuePairDe<'de> for KVList<T, U, V>
where
    T: serde::de::Deserializer<'de, Error = serde::de::value::Error>,
    U: serde::de::Deserializer<'de, Error = serde::de::value::Error>,
    V: KeyValuePairDe<'de>,
{
    fn key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, serde::de::value::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        if let Some(t) = self.first.take() {
            match t {
                KV::Pair(k, v) => {
                    self.first = Some(KV::V(v));
                    seed.deserialize(k).map(Some)
                }
                KV::V(_) => Err(<serde::de::value::Error as serde::de::Error>::custom(
                    "should not call next_key when expecting a value",
                )),
            }
        } else {
            self.second.key_seed(seed)
        }
    }

    fn value_seed<W>(&mut self, seed: W) -> Result<W::Value, serde::de::value::Error>
    where
        W: serde::de::DeserializeSeed<'de>,
    {
        if let Some(t) = self.first.take() {
            match t {
                KV::V(v) => seed.deserialize(v),
                KV::Pair(..) => Err(<serde::de::value::Error as serde::de::Error>::custom(
                    "should not call next_value when expecting a key",
                )),
            }
        } else {
            self.second.value_seed(seed)
        }
    }
}
impl<T, U, V> KeyValuePairSer for KVList<T, U, V>
where
    T: serde::ser::Serialize,
    U: serde::ser::Serialize,
    V: KeyValuePairSer,
{
    #[inline]
    fn serialize<S>(&self, seq: &mut S) -> Result<(), S::Error>
    where
        S: serde::ser::SerializeMap,
    {
        if let Some(KV::Pair(k, v)) = &self.first {
            seq.serialize_key(k)?;
            seq.serialize_value(v)?;
        }
        self.second.serialize(seq)?;
        Ok(())
    }

    #[inline]
    fn size(&self) -> usize {
        1 + self.second.size()
    }
}

impl<'de> KeyValuePairDe<'de> for () {
    fn key_seed<K>(&mut self, _seed: K) -> Result<Option<K::Value>, serde::de::value::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        Ok(None)
    }

    fn value_seed<V>(&mut self, _seed: V) -> Result<V::Value, serde::de::value::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        Err(<serde::de::value::Error as serde::de::Error>::custom(
            "should not call next_value when expecting a key",
        ))
    }
}
impl KeyValuePairSer for () {
    #[inline]
    fn serialize<S>(&self, _seq: &mut S) -> Result<(), S::Error>
    where
        S: serde::ser::SerializeMap,
    {
        Ok(())
    }

    #[inline]
    fn size(&self) -> usize {
        0
    }
}

trait KeyValuePairDe<'de> {
    fn key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, serde::de::value::Error>
    where
        K: serde::de::DeserializeSeed<'de>;

    fn value_seed<V>(&mut self, seed: V) -> Result<V::Value, serde::de::value::Error>
    where
        V: serde::de::DeserializeSeed<'de>;
}
trait KeyValuePairSer {
    fn serialize<S>(&self, seq: &mut S) -> Result<(), S::Error>
    where
        S: serde::ser::SerializeMap;
    fn size(&self) -> usize;
}

#[derive(Copy, Clone)]
#[doc(hidden)]
pub struct Map<T>(pub T);

struct MapState<T>(T);

impl<'de, T: KeyValuePairDe<'de>> crate::Deserializer<'de> for Map<T> {
    fn deserialize_any2<V>(self, visitor: V) -> Result<V::Value, serde::de::value::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_map(MapState(self.0))
    }
}
impl<'de, T: KeyValuePairDe<'de>> serde::de::Deserializer<'de> for Map<T> {
    type Error = serde::de::value::Error;
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_map(MapState(self.0))
    }
    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}
impl<'de, T: KeyValuePairDe<'de>> serde::de::MapAccess<'de> for MapState<T> {
    type Error = serde::de::value::Error;
    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        self.0.key_seed(seed)
    }
    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        self.0.value_seed(seed)
    }
}
impl<T: KeyValuePairSer> serde::ser::Serialize for Map<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_map(Some(self.0.size()))?;
        self.0.serialize(&mut seq)?;
        serde::ser::SerializeMap::end(seq)
    }
}

#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct List1<T, U> {
    pub first: Option<T>,
    pub second: U,
}

impl<'de, T, U> ItemDe<'de> for List1<T, U>
where
    T: serde::de::Deserializer<'de, Error = serde::de::value::Error>,
    U: ItemDe<'de>,
{
    fn value_seed<V>(&mut self, seed: V) -> Result<Option<V::Value>, serde::de::value::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        if let Some(t) = self.first.take() {
            seed.deserialize(t).map(Some)
        } else {
            self.second.value_seed(seed)
        }
    }
}

impl<T, U> ItemSer for List1<T, U>
where
    T: serde::ser::Serialize,
    U: ItemSer,
{
    #[inline]
    fn serialize<S>(&self, seq: &mut S) -> Result<(), S::Error>
    where
        S: serde::ser::SerializeSeq,
    {
        if let Some(t) = &self.first {
            seq.serialize_element(t)?;
        }
        self.second.serialize(seq)?;
        Ok(())
    }

    #[inline]
    fn size(&self) -> usize {
        1 + self.second.size()
    }
}

impl<'de> ItemDe<'de> for () {
    fn value_seed<V>(&mut self, _seed: V) -> Result<Option<V::Value>, serde::de::value::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        Ok(None)
    }
}
impl ItemSer for () {
    #[inline]
    fn serialize<S>(&self, _seq: &mut S) -> Result<(), S::Error>
    where
        S: serde::ser::SerializeSeq,
    {
        Ok(())
    }

    #[inline]
    fn size(&self) -> usize {
        0
    }
}

trait ItemDe<'de> {
    fn value_seed<V>(&mut self, seed: V) -> Result<Option<V::Value>, serde::de::value::Error>
    where
        V: serde::de::DeserializeSeed<'de>;
}

trait ItemSer {
    fn serialize<S>(&self, seq: &mut S) -> Result<(), S::Error>
    where
        S: serde::ser::SerializeSeq;
    fn size(&self) -> usize;
}
#[derive(Copy, Clone)]
#[doc(hidden)]
pub struct List<T>(pub T);

struct ListState<T>(T);

impl<'de, T: ItemDe<'de>> crate::Deserializer<'de> for List<T> {
    fn deserialize_any2<V>(self, visitor: V) -> Result<V::Value, serde::de::value::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_seq(ListState(self.0))
    }
}
impl<'de, T: ItemDe<'de>> serde::de::Deserializer<'de> for List<T> {
    type Error = serde::de::value::Error;
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_seq(ListState(self.0))
    }
    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}
impl<'de, K: ItemDe<'de>> serde::de::SeqAccess<'de> for ListState<K> {
    type Error = serde::de::value::Error;
    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        self.0.value_seed(seed)
    }
}
impl<T: ItemSer> serde::ser::Serialize for List<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.0.size()))?;
        self.0.serialize(&mut seq)?;
        serde::ser::SerializeSeq::end(seq)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use serde::Deserialize;
    use serde_test::Token;

    #[derive(Debug, Deserialize)]
    struct Something {
        foo: i32,
    }

    #[test]
    fn object() {
        // FIXME: `zst_expr()` breaks `Copy`.
        let data = || json!({"foo": 123});
        let x = Something::deserialize(data()).unwrap();
        let y = <BTreeMap<String, i32>>::deserialize(data()).unwrap();
        assert_eq!(x.foo, 123);
        assert_eq!(y["foo"], 123);
    }

    #[test]
    fn object_ser() {
        serde_test::assert_ser_tokens(
            &json!({"foo": 123}),
            &[
                Token::Map { len: Some(1) },
                Token::Str("foo"),
                Token::I32(123),
                Token::MapEnd,
            ],
        );
    }

    #[test]
    fn array() {
        let data = json!([123, 456]);
        let x = <[i32; 2]>::deserialize(data).unwrap();
        let y = <Vec<i32>>::deserialize(data).unwrap();
        assert_eq!(x, [123, 456]);
        assert_eq!(y, [123, 456]);
    }

    #[test]
    fn array_ser() {
        serde_test::assert_ser_tokens(
            &json!([123, 456]),
            &[
                Token::Seq { len: Some(2) },
                Token::I32(123),
                Token::I32(456),
                Token::SeqEnd,
            ],
        );
    }

    #[test]
    fn complex_ser() {
        let value1 = 123;
        let value2 = 456;
        let value3 = format!("hello {}", "world");

        let data = json!({
            "codes": [value1, value2],
            "message": value3
        });

        serde_test::assert_ser_tokens(
            &data,
            &[
                Token::Map { len: Some(2) },
                Token::Str("codes"),
                Token::Seq { len: Some(2) },
                Token::I32(123),
                Token::I32(456),
                Token::SeqEnd,
                Token::Str("message"),
                Token::Str("hello world"),
                Token::MapEnd,
            ],
        );
    }
}
