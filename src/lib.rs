use serde::de::*;
use serde::forward_to_deserialize_any;

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

// -- user code --

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    struct Something {
        foo: i32,
    }

    #[test]
    fn main() {
        let data = json!({"foo": 123});
        let x = Something::deserialize(data).unwrap();
        let y = <BTreeMap<&'static str, i32>>::deserialize(data).unwrap();
        assert_eq!(x.foo, 123);
        assert_eq!(y["foo"], 123);
    }
}
