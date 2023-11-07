use serde::de::*;
use serde::forward_to_deserialize_any;
use serde::ser::SerializeMap;
use serde::ser::SerializeSeq;

#[macro_use]
mod macros;

// -- library code --

struct Lit<T>(T);
impl<'de> Deserializer<'de> for Lit<i64> {
    type Error = serde::de::value::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(self.0)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

impl<'de> Deserializer<'de> for Lit<&'static str> {
    type Error = serde::de::value::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_borrowed_str(self.0)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

impl serde::ser::Serialize for Lit<i64> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i64(self.0)
    }
}

impl serde::ser::Serialize for Lit<&'static str> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.0)
    }
}

#[derive(Copy, Clone)]
pub struct EmptyMap;
struct EmptyMapState;

impl<'de> serde::de::Deserializer<'de> for EmptyMap {
    type Error = serde::de::value::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_map(EmptyMapState)
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

impl<'de> serde::de::MapAccess<'de> for EmptyMapState {
    type Error = serde::de::value::Error;

    fn next_key_seed<K>(&mut self, _seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        Ok(None)
    }
    fn next_value_seed<V>(&mut self, _seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        Err(serde::de::value::Error::custom("foo"))
    }
}

impl serde::ser::Serialize for EmptyMap {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_map(Some(0))?.end()
    }
}

#[derive(Copy, Clone)]
pub struct EmptyList;
struct EmptyListState;

impl<'de> serde::de::Deserializer<'de> for EmptyList {
    type Error = serde::de::value::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_seq(EmptyListState)
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

impl<'de> serde::de::SeqAccess<'de> for EmptyListState {
    type Error = serde::de::value::Error;

    fn next_element_seed<T>(&mut self, _seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        Ok(None)
    }
}

impl serde::ser::Serialize for EmptyList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_seq(Some(0))?.end()
    }
}

struct Null;

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

struct Bool(bool);

impl<'de> serde::de::Deserializer<'de> for Bool {
    type Error = serde::de::value::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_bool(self.0)
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

impl serde::ser::Serialize for Bool {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bool(self.0)
    }
}

#[doc(hidden)]
pub enum KV<T, U> {
    Pair(T, U),
    V(U),
    Empty,
}

impl<'de, T, U> KeyValuePair<'de> for KV<T, U>
where
    T: serde::ser::Serialize + serde::de::Deserializer<'de, Error = serde::de::value::Error>,
    U: serde::ser::Serialize + serde::de::Deserializer<'de, Error = serde::de::value::Error>,
{
    fn key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, serde::de::value::Error>
    where
        K: DeserializeSeed<'de>,
    {
        match std::mem::replace(self, KV::Empty) {
            KV::Pair(k, v) => {
                *self = KV::V(v);
                seed.deserialize(k).map(Some)
            }
            KV::Empty => Ok(None),
            _ => Err(<serde::de::value::Error as serde::de::Error>::custom(
                "foobar",
            )),
        }
    }

    fn value_seed<V>(&mut self, seed: V) -> Result<V::Value, serde::de::value::Error>
    where
        V: DeserializeSeed<'de>,
    {
        match std::mem::replace(self, KV::Empty) {
            KV::V(v) => seed.deserialize(v),
            _ => Err(<serde::de::value::Error as serde::de::Error>::custom(
                "foobar",
            )),
        }
    }

    fn serialize<S>(&self, seq: &mut S) -> Result<(), S::Error>
    where
        S: serde::ser::SerializeMap,
    {
        match self {
            KV::Pair(k, v) => {
                serde::ser::SerializeMap::serialize_key(seq, &k)?;
                serde::ser::SerializeMap::serialize_value(seq, &v)?;
                Ok(())
            }
            _ => Err(<S::Error as serde::ser::Error>::custom("foobar")),
        }
    }

    fn len(&self) -> usize {
        1
    }
}

impl<'de> KeyValuePair<'de> for () {
    fn key_seed<K>(&mut self, _seed: K) -> Result<Option<K::Value>, serde::de::value::Error>
    where
        K: DeserializeSeed<'de>,
    {
        Ok(None)
    }

    fn value_seed<V>(&mut self, _seed: V) -> Result<V::Value, serde::de::value::Error>
    where
        V: DeserializeSeed<'de>,
    {
        Err(<serde::de::value::Error as serde::de::Error>::custom(
            "foobar",
        ))
    }

    fn serialize<S>(&self, _seq: &mut S) -> Result<(), S::Error>
    where
        S: serde::ser::SerializeMap,
    {
        Ok(())
    }

    fn len(&self) -> usize {
        0
    }
}

impl<'de> Item<'de> for () {
    fn value_seed<V>(&mut self, _seed: V) -> Result<Option<V::Value>, serde::de::value::Error>
    where
        V: DeserializeSeed<'de>,
    {
        Ok(None)
    }

    fn serialize<S>(&self, _seq: &mut S) -> Result<(), S::Error>
    where
        S: serde::ser::SerializeSeq,
    {
        Ok(())
    }

    fn len(&self) -> usize {
        0
    }
}

#[doc(hidden)]
pub struct KVList<T, U> {
    state: bool,
    first: T,
    second: U,
}

impl<'de, T, U> KeyValuePair<'de> for KVList<T, U>
where
    T: KeyValuePair<'de>,
    U: KeyValuePair<'de>,
{
    fn key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, serde::de::value::Error>
    where
        K: DeserializeSeed<'de>,
    {
        if self.state {
            self.first.key_seed(seed)
        } else {
            self.second.key_seed(seed)
        }
    }

    fn value_seed<V>(&mut self, seed: V) -> Result<V::Value, serde::de::value::Error>
    where
        V: DeserializeSeed<'de>,
    {
        if self.state {
            self.state = false;
            self.first.value_seed(seed)
        } else {
            self.second.value_seed(seed)
        }
    }

    fn serialize<S>(&self, seq: &mut S) -> Result<(), S::Error>
    where
        S: serde::ser::SerializeMap,
    {
        self.first.serialize(seq)?;
        self.second.serialize(seq)?;
        Ok(())
    }

    fn len(&self) -> usize {
        self.first.len() + self.second.len()
    }
}

pub trait KeyValuePair<'de> {
    fn key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, serde::de::value::Error>
    where
        K: DeserializeSeed<'de>;

    fn value_seed<V>(&mut self, seed: V) -> Result<V::Value, serde::de::value::Error>
    where
        V: DeserializeSeed<'de>;

    fn serialize<S>(&self, seq: &mut S) -> Result<(), S::Error>
    where
        S: serde::ser::SerializeMap;
    fn len(&self) -> usize;
}

#[derive(Copy, Clone)]
struct Map<T>(T);

struct MapState<T>(T);

impl<'de, T: KeyValuePair<'de>> serde::de::Deserializer<'de> for Map<T> {
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
impl<'de, T: KeyValuePair<'de>> serde::de::MapAccess<'de> for MapState<T> {
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
impl<T: for<'de> KeyValuePair<'de>> serde::ser::Serialize for Map<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_map(Some(self.0.len()))?;
        self.0.serialize(&mut seq)?;
        serde::ser::SerializeMap::end(seq)
    }
}

impl<'de, T> Item<'de> for Option<T>
where
    T: serde::ser::Serialize + serde::de::Deserializer<'de, Error = serde::de::value::Error>,
{
    fn value_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, serde::de::value::Error>
    where
        K: DeserializeSeed<'de>,
    {
        let Some(value) = self.take() else {
            return Ok(None);
        };
        seed.deserialize(value).map(Some)
    }

    fn serialize<S>(&self, seq: &mut S) -> Result<(), S::Error>
    where
        S: serde::ser::SerializeSeq,
    {
        match self {
            Some(v) => {
                serde::ser::SerializeSeq::serialize_element(seq, &v)?;
                Ok(())
            }
            _ => Err(<S::Error as serde::ser::Error>::custom("foobar")),
        }
    }

    fn len(&self) -> usize {
        1
    }
}

#[doc(hidden)]
pub struct List1<T, U> {
    first: Option<T>,
    second: U,
}

impl<'de, T, U> Item<'de> for List1<T, U>
where
    T: serde::ser::Serialize + serde::de::Deserializer<'de, Error = serde::de::value::Error>,
    U: Item<'de>,
{
    fn value_seed<V>(&mut self, seed: V) -> Result<Option<V::Value>, serde::de::value::Error>
    where
        V: DeserializeSeed<'de>,
    {
        if let Some(t) = self.first.take() {
            seed.deserialize(t).map(Some)
        } else {
            self.second.value_seed(seed)
        }
    }

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

    fn len(&self) -> usize {
        1 + self.second.len()
    }
}

pub trait Item<'de> {
    fn value_seed<V>(&mut self, seed: V) -> Result<Option<V::Value>, serde::de::value::Error>
    where
        V: DeserializeSeed<'de>;

    fn serialize<S>(&self, seq: &mut S) -> Result<(), S::Error>
    where
        S: serde::ser::SerializeSeq;
    fn len(&self) -> usize;
}

#[derive(Copy, Clone)]
struct List<T>(T);

struct ListState<T>(T);

impl<'de, T: Item<'de>> serde::de::Deserializer<'de> for List<T> {
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
impl<'de, K: Item<'de>> serde::de::SeqAccess<'de> for ListState<K> {
    type Error = serde::de::value::Error;
    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        self.0.value_seed(seed)
    }
}
impl<T: for<'de> Item<'de>> serde::ser::Serialize for List<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        self.0.serialize(&mut seq)?;
        serde::ser::SerializeSeq::end(seq)
    }
}

// -- user code --

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};

    use serde::Deserialize;
    use serde_test::Token;

    use crate::{KVList, List, List1, Lit, Map, KV};

    #[derive(Debug, Deserialize)]
    struct Something {
        foo: i32,
    }

    #[test]
    fn arbitrary_map() {
        let data = Map(KVList {
            state: true,
            first: KV::Pair(Lit("foo"), Lit(1)),
            second: KVList {
                state: true,
                first: KV::Pair(Lit("bar"), Lit(2)),
                second: KV::Pair(Lit("baz"), Lit(3)),
            },
        });
        serde_test::assert_ser_tokens(
            &data,
            &[
                Token::Map { len: Some(3) },
                Token::Str("foo"),
                Token::I64(1),
                Token::Str("bar"),
                Token::I64(2),
                Token::Str("baz"),
                Token::I64(3),
                Token::MapEnd,
            ],
        );
        let y = <BTreeMap<&'static str, i32>>::deserialize(data).unwrap();
        dbg!(y);
    }

    #[test]
    fn arbitrary_seq() {
        let data = List(List1 {
            first: Some(Lit("foo")),
            second: List1 {
                first: Some(Lit("bar")),
                second: List1 {
                    first: Some(Lit("baz")),
                    second: (),
                },
            },
        });
        serde_test::assert_ser_tokens(
            &data,
            &[
                Token::Seq { len: Some(3) },
                Token::Str("foo"),
                Token::Str("bar"),
                Token::Str("baz"),
                Token::SeqEnd,
            ],
        );
        let y = <BTreeSet<&'static str>>::deserialize(data).unwrap();
        dbg!(y);
    }

    #[test]
    fn object() {
        let data = json!({"foo": 123});
        let x = Something::deserialize(data).unwrap();
        let y = <BTreeMap<&'static str, i32>>::deserialize(data).unwrap();
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
                Token::I64(123),
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
                Token::I64(123),
                Token::I64(456),
                Token::SeqEnd,
            ],
        );
    }
}
