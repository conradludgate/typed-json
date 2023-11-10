use crate::DeShared;

impl<'de, T> ItemDe<'de> for Option<T>
where
    T: serde::de::Deserializer<'de, Error = serde::de::value::Error>,
{
    fn value_seed<V>(&mut self, seed: V) -> Result<Option<V::Value>, serde::de::value::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        if let Some(t) = self.take() {
            seed.deserialize(t).map(Some)
        } else {
            Ok(None)
        }
    }
}

impl<T> ItemSer for Option<T>
where
    T: serde::ser::Serialize,
{
    fn serialize<S>(&self, seq: &mut S) -> Result<(), S::Error>
    where
        S: serde::ser::SerializeSeq,
    {
        if let Some(x) = self {
            seq.serialize_element(x)?;
        }
        Ok(())
    }

    fn size(&self) -> usize {
        1
    }
}

impl<'de, T, U> ItemDe<'de> for (T, U)
where
    T: ItemDe<'de>,
    U: ItemDe<'de>,
{
    fn value_seed<V>(&mut self, seed: V) -> Result<Option<V::Value>, serde::de::value::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        if !self.0.is_done() {
            self.0.value_seed(seed)
        } else {
            self.1.value_seed(seed)
        }
    }
}

impl<T, U> ItemSer for (T, U)
where
    T: ItemSer,
    U: ItemSer,
{
    #[inline]
    fn serialize<S>(&self, seq: &mut S) -> Result<(), S::Error>
    where
        S: serde::ser::SerializeSeq,
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

trait ItemDe<'de>: DeShared {
    fn value_seed<V>(&mut self, seed: V) -> Result<Option<V::Value>, serde::de::value::Error>
    where
        V: serde::de::DeserializeSeed<'de>;
}

pub trait ItemSer {
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
