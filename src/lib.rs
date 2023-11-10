#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[macro_use]
mod macros;
mod expr_de;

#[cfg(feature = "std")]
mod fmt;

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

impl<'de, T, U> KeyValuePairDe<'de> for Option2<KV<T, U>>
where
    T: serde::de::Deserializer<'de, Error = serde::de::value::Error>,
    U: serde::de::Deserializer<'de, Error = serde::de::value::Error>,
{
    fn key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, serde::de::value::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        match core::mem::replace(self, Option2::None) {
            Option2::Some(KV::Pair(k, v)) => {
                *self = Option2::Some(KV::V(v));
                seed.deserialize(k).map(Some)
            }
            Option2::Some(KV::V(_)) => Err(<serde::de::value::Error as serde::de::Error>::custom(
                "should not call next_key when expecting a value",
            )),
            Option2::None => Ok(None),
        }
    }

    fn value_seed<W>(&mut self, seed: W) -> Result<W::Value, serde::de::value::Error>
    where
        W: serde::de::DeserializeSeed<'de>,
    {
        match core::mem::replace(self, Option2::None) {
            Option2::Some(KV::Pair(..)) => {
                Err(<serde::de::value::Error as serde::de::Error>::custom(
                    "should not call next_value when expecting a key",
                ))
            }
            Option2::Some(KV::V(v)) => seed.deserialize(v),
            Option2::None => {
                unimplemented!()
            }
        }
    }
}

impl<T, U> KeyValuePairSer for HList<T, U>
where
    T: KeyValuePairSer,
    U: KeyValuePairSer,
{
    #[inline]
    fn serialize<S>(&self, seq: &mut S) -> Result<(), S::Error>
    where
        S: serde::ser::SerializeMap,
    {
        self.first.serialize(seq)?;
        self.second.serialize(seq)?;
        Ok(())
    }

    #[inline]
    fn size(&self) -> usize {
        self.first.size() + self.second.size()
    }
}

impl<T, U> KeyValuePairSer for Option2<KV<T, U>>
where
    T: serde::ser::Serialize,
    U: serde::ser::Serialize,
{
    #[inline]
    fn serialize<S>(&self, seq: &mut S) -> Result<(), S::Error>
    where
        S: serde::ser::SerializeMap,
    {
        if let Option2::Some(KV::Pair(k, v)) = self {
            seq.serialize_key(k)?;
            seq.serialize_value(v)?;
        }
        Ok(())
    }

    #[inline]
    fn size(&self) -> usize {
        1
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

trait KeyValuePairDe<'de>: DeShared {
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
pub enum Option2<T> {
    Some(T),
    None,
}

impl<T> DeShared for Option2<T> {
    fn is_done(&self) -> bool {
        matches!(self, Option2::None)
    }
}

impl<'de, T> ItemDe<'de> for Option2<T>
where
    T: serde::de::Deserializer<'de, Error = serde::de::value::Error>,
{
    fn value_seed<V>(&mut self, seed: V) -> Result<Option<V::Value>, serde::de::value::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        if let Option2::Some(t) = core::mem::replace(self, Option2::None) {
            seed.deserialize(t).map(Some)
        } else {
            Ok(None)
        }
    }
}
impl<T> ItemSer for Option2<T>
where
    T: serde::ser::Serialize,
{
    fn serialize<S>(&self, seq: &mut S) -> Result<(), S::Error>
    where
        S: serde::ser::SerializeSeq,
    {
        if let Option2::Some(x) = self {
            seq.serialize_element(x)?;
        }
        Ok(())
    }

    fn size(&self) -> usize {
        1
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

impl<'de, T, U> ItemDe<'de> for HList<T, U>
where
    T: ItemDe<'de>,
    U: ItemDe<'de>,
{
    fn value_seed<V>(&mut self, seed: V) -> Result<Option<V::Value>, serde::de::value::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        if !self.first.is_done() {
            self.first.value_seed(seed)
        } else {
            self.second.value_seed(seed)
        }
    }
}

impl<T, U> ItemSer for HList<T, U>
where
    T: ItemSer,
    U: ItemSer,
{
    #[inline]
    fn serialize<S>(&self, seq: &mut S) -> Result<(), S::Error>
    where
        S: serde::ser::SerializeSeq,
    {
        self.first.serialize(seq)?;
        self.second.serialize(seq)?;
        Ok(())
    }

    #[inline]
    fn size(&self) -> usize {
        self.first.size() + self.second.size()
    }
}

impl DeShared for () {
    fn is_done(&self) -> bool {
        true
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

trait DeShared {
    fn is_done(&self) -> bool;
}

trait ItemDe<'de>: DeShared {
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
pub struct Array<T>(pub T);

struct ListState<T>(T);

impl<'de, T: ItemDe<'de>> crate::Deserializer<'de> for Array<T> {
    fn deserialize_any2<V>(self, visitor: V) -> Result<V::Value, serde::de::value::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_seq(ListState(self.0))
    }
}
impl<'de, T: ItemDe<'de>> serde::de::Deserializer<'de> for Array<T> {
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
impl<T: ItemSer> serde::ser::Serialize for Array<T> {
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

    use crate::Option2;

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

    #[test]
    fn example() {
        let input = 1;
        let data = crate::Map(crate::HList {
            first: Option2::Some(crate::KV::Pair(crate::Expr("foo"), crate::Expr(input))),
            second: crate::HList {
                first: Option2::Some(crate::KV::Pair(
                    crate::Expr("bar"),
                    crate::Array(Option2::Some(crate::Expr(input))),
                )),
                second: Option2::Some(crate::KV::Pair(
                    crate::Expr("baz"),
                    crate::Map(crate::HList {
                        first: Option2::Some(crate::KV::Pair(
                            crate::Expr("code"),
                            crate::Expr(input),
                        )),
                        second: crate::HList {
                            first: Option2::Some(crate::KV::Pair(
                                crate::Expr("extra"),
                                crate::Null,
                            )),
                            second: Option2::Some(crate::KV::Pair(
                                crate::Expr("this"),
                                crate::Map(Option2::Some(crate::KV::Pair(
                                    crate::Expr("is"),
                                    crate::Map(Option2::Some(crate::KV::Pair(
                                        crate::Expr("a"),
                                        crate::Array(crate::HList {
                                            first: Option2::Some(crate::Expr(input)),
                                            second: Option2::Some(crate::Map(Option2::Some(
                                                crate::KV::Pair(
                                                    crate::Expr("really"),
                                                    crate::Map(Option2::Some(crate::KV::Pair(
                                                        crate::Expr("deep"),
                                                        crate::Array(crate::HList {
                                                            first: Option2::Some(crate::Expr(
                                                                "object",
                                                            )),
                                                            second: crate::HList {
                                                                first: crate::HList {
                                                                    first: Option2::Some(
                                                                        crate::Expr(input),
                                                                    ),
                                                                    second: Option2::Some(
                                                                        crate::Null,
                                                                    ),
                                                                },
                                                                second: crate::HList {
                                                                    first: Option2::Some(
                                                                        crate::Expr(true),
                                                                    ),
                                                                    second: Option2::Some(
                                                                        crate::Expr(false),
                                                                    ),
                                                                },
                                                            },
                                                        }),
                                                    ))),
                                                ),
                                            ))),
                                        }),
                                    ))),
                                ))),
                            )),
                        },
                    }),
                )),
            },
        });

        assert_eq!(serde_json::to_string(&data).unwrap(), "{\"foo\":1,\"bar\":[1],\"baz\":{\"code\":1,\"extra\":null,\"this\":{\"is\":{\"a\":[1,{\"really\":{\"deep\":[\"object\",1,null,true,false]}}]}}}}");
    }
}
