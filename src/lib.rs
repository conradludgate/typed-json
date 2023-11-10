#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

use expr_de::Expr;

#[macro_use]
mod macros;
mod expr_de;

#[cfg(feature = "std")]
mod fmt;

mod array;
mod map;

#[doc(hidden)]
pub mod __private {
    pub use crate::array::Array;
    pub use crate::expr_de::Expr;
    pub use crate::map::{Map, KV};
    pub use crate::HList;
    pub use crate::Null;
}

/// A clone of [`serde::de::Deserializer`] to get around the orphan rule
pub trait Deserializer<'de>: Sized {
    /// Require the `Deserializer` to figure out how to drive the visitor based
    /// on what data type is in the input.
    fn deserialize_any2<V>(self, visitor: V) -> Result<V::Value, serde::de::value::Error>
    where
        V: serde::de::Visitor<'de>;
}

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

trait DeShared {
    fn is_done(&self) -> bool;
}

impl<T> DeShared for Option<T> {
    fn is_done(&self) -> bool {
        self.is_none()
    }
}

#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct HList<T, U> {
    pub first: T,
    pub second: U,
}

impl<T, U> DeShared for HList<T, U>
where
    T: DeShared,
    U: DeShared,
{
    fn is_done(&self) -> bool {
        self.first.is_done() && self.second.is_done()
    }
}

impl DeShared for () {
    fn is_done(&self) -> bool {
        true
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
        let data = json!({"foo": 123});
        let x = Something::deserialize(data).unwrap();
        let y = <BTreeMap<String, i32>>::deserialize(data).unwrap();
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
