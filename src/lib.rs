use serde::de::*;
use serde::forward_to_deserialize_any;

#[macro_use]
mod macros;

// -- library code --

#[derive(Clone, Copy)]
#[doc(hidden)]
pub struct Expr<T>(pub T);
impl<'de> Deserializer<'de> for Expr<i64> {
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

impl<'de> Deserializer<'de> for Expr<&str> {
    type Error = serde::de::value::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_str(self.0)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

impl<'de> Deserializer<'de> for Expr<String> {
    type Error = serde::de::value::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_string(self.0)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

impl serde::ser::Serialize for Expr<i64> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i64(self.0)
    }
}

impl serde::ser::Serialize for Expr<&str> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.0)
    }
}

impl serde::ser::Serialize for Expr<String> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

#[derive(Clone, Copy)]
#[doc(hidden)]
pub struct Null;

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

impl<'de> serde::de::Deserializer<'de> for Expr<bool> {
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

impl serde::ser::Serialize for Expr<bool> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bool(self.0)
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

impl<'de, T, U, V> KeyValuePair<'de> for KVList<T, U, V>
where
    T: serde::ser::Serialize + serde::de::Deserializer<'de, Error = serde::de::value::Error>,
    U: serde::ser::Serialize + serde::de::Deserializer<'de, Error = serde::de::value::Error>,
    V: KeyValuePair<'de>,
{
    fn key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, serde::de::value::Error>
    where
        K: DeserializeSeed<'de>,
    {
        if let Some(t) = self.first.take() {
            match t {
                KV::Pair(k, v) => {
                    self.first = Some(KV::V(v));
                    seed.deserialize(k).map(Some)
                }
                KV::V(_) => Err(<serde::de::value::Error as serde::de::Error>::custom(
                    "foobar",
                )),
            }
        } else {
            self.second.key_seed(seed)
        }
    }

    fn value_seed<W>(&mut self, seed: W) -> Result<W::Value, serde::de::value::Error>
    where
        W: DeserializeSeed<'de>,
    {
        if let Some(t) = self.first.take() {
            match t {
                KV::V(v) => seed.deserialize(v),
                KV::Pair(..) => Err(<serde::de::value::Error as serde::de::Error>::custom(
                    "foobar",
                )),
            }
        } else {
            self.second.value_seed(seed)
        }
    }

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

    fn size(&self) -> usize {
        1 + self.second.size()
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

    fn size(&self) -> usize {
        0
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
    fn size(&self) -> usize;
}

#[derive(Copy, Clone)]
#[doc(hidden)]
pub struct Map<T>(pub T);

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

    fn size(&self) -> usize {
        1 + self.second.size()
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

    fn size(&self) -> usize {
        0
    }
}

pub trait Item<'de> {
    fn value_seed<V>(&mut self, seed: V) -> Result<Option<V::Value>, serde::de::value::Error>
    where
        V: DeserializeSeed<'de>;

    fn serialize<S>(&self, seq: &mut S) -> Result<(), S::Error>
    where
        S: serde::ser::SerializeSeq;
    fn size(&self) -> usize;
}

#[derive(Copy, Clone)]
#[doc(hidden)]
pub struct List<T>(pub T);

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
        let mut seq = serializer.serialize_seq(Some(self.0.size()))?;
        self.0.serialize(&mut seq)?;
        serde::ser::SerializeSeq::end(seq)
    }
}

// -- user code --

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
                Token::I64(123),
                Token::I64(456),
                Token::SeqEnd,
                Token::Str("message"),
                Token::Str("hello world"),
                Token::MapEnd,
            ],
        );
    }
}
