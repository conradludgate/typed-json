use serde::{de::Visitor, forward_to_deserialize_any, Deserializer};

#[derive(Clone, Copy)]
pub struct Expr<T>(pub T);

impl<'de, D: crate::Deserializer<'de>> Deserializer<'de> for Expr<D> {
    type Error = serde::de::value::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.deserialize_any2(visitor)
    }

    #[inline]
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_some(self)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}
impl<'de, D: crate::Deserializer<'de>> crate::Deserializer<'de> for Expr<D> {
    fn deserialize_any2<V>(self, visitor: V) -> Result<V::Value, serde::de::value::Error>
    where
        V: Visitor<'de>,
    {
        self.0.deserialize_any2(visitor)
    }
}

impl<'de> crate::Deserializer<'de> for i128 {
    fn deserialize_any2<V>(self, visitor: V) -> Result<V::Value, serde::de::value::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i128(self)
    }
}
impl<'de> crate::Deserializer<'de> for i64 {
    fn deserialize_any2<V>(self, visitor: V) -> Result<V::Value, serde::de::value::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(self)
    }
}
impl<'de> crate::Deserializer<'de> for i32 {
    fn deserialize_any2<V>(self, visitor: V) -> Result<V::Value, serde::de::value::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i32(self)
    }
}
impl<'de> crate::Deserializer<'de> for i16 {
    fn deserialize_any2<V>(self, visitor: V) -> Result<V::Value, serde::de::value::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i16(self)
    }
}
impl<'de> crate::Deserializer<'de> for i8 {
    fn deserialize_any2<V>(self, visitor: V) -> Result<V::Value, serde::de::value::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i8(self)
    }
}
impl<'de> crate::Deserializer<'de> for u128 {
    fn deserialize_any2<V>(self, visitor: V) -> Result<V::Value, serde::de::value::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u128(self)
    }
}
impl<'de> crate::Deserializer<'de> for u64 {
    fn deserialize_any2<V>(self, visitor: V) -> Result<V::Value, serde::de::value::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(self)
    }
}
impl<'de> crate::Deserializer<'de> for u32 {
    fn deserialize_any2<V>(self, visitor: V) -> Result<V::Value, serde::de::value::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(self)
    }
}
impl<'de> crate::Deserializer<'de> for u16 {
    fn deserialize_any2<V>(self, visitor: V) -> Result<V::Value, serde::de::value::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u16(self)
    }
}
impl<'de> crate::Deserializer<'de> for u8 {
    fn deserialize_any2<V>(self, visitor: V) -> Result<V::Value, serde::de::value::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u8(self)
    }
}

impl<'de> crate::Deserializer<'de> for &str {
    fn deserialize_any2<V>(self, visitor: V) -> Result<V::Value, serde::de::value::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_str(self)
    }
}

#[cfg(feature = "std")]
impl<'de> crate::Deserializer<'de> for String {
    fn deserialize_any2<V>(self, visitor: V) -> Result<V::Value, serde::de::value::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_string(self)
    }
}

impl<'de> crate::Deserializer<'de> for bool {
    fn deserialize_any2<V>(self, visitor: V) -> Result<V::Value, serde::de::value::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_bool(self)
    }
}
