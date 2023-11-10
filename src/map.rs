use crate::DeShared;

#[doc(hidden)]
#[derive(Clone, Copy)]
pub enum KV<T, U> {
    Pair(T, U),
    V(U),
}

impl<'de, T, U> KeyValuePairDe<'de> for Option<KV<T, U>>
where
    T: serde::de::Deserializer<'de, Error = serde::de::value::Error>,
    U: serde::de::Deserializer<'de, Error = serde::de::value::Error>,
{
    fn key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, serde::de::value::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        match self.take() {
            Some(KV::Pair(k, v)) => {
                *self = Some(KV::V(v));
                seed.deserialize(k).map(Some)
            }
            Some(KV::V(_)) => Err(<serde::de::value::Error as serde::de::Error>::custom(
                "should not call next_key when expecting a value",
            )),
            None => Ok(None),
        }
    }

    fn value_seed<W>(&mut self, seed: W) -> Result<W::Value, serde::de::value::Error>
    where
        W: serde::de::DeserializeSeed<'de>,
    {
        match self.take() {
            Some(KV::Pair(..)) => Err(<serde::de::value::Error as serde::de::Error>::custom(
                "should not call next_value when expecting a key",
            )),
            Some(KV::V(v)) => seed.deserialize(v),
            None => {
                unimplemented!()
            }
        }
    }
}

impl<T, U> KeyValuePairSer for (T, U)
where
    T: KeyValuePairSer,
    U: KeyValuePairSer,
{
    #[inline]
    fn serialize<S>(&self, seq: &mut S) -> Result<(), S::Error>
    where
        S: serde::ser::SerializeMap,
    {
        self.0.serialize(seq)?;
        self.1.serialize(seq)?;
        Ok(())
    }

    #[inline]
    fn size(&self) -> usize {
        self.0.size() + self.1.size()
    }
}

impl<T, U> KeyValuePairSer for Option<KV<T, U>>
where
    T: serde::ser::Serialize,
    U: serde::ser::Serialize,
{
    #[inline]
    fn serialize<S>(&self, seq: &mut S) -> Result<(), S::Error>
    where
        S: serde::ser::SerializeMap,
    {
        if let Some(KV::Pair(k, v)) = self {
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
pub trait KeyValuePairSer {
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
