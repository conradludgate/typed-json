#[macro_export(local_inner_macros)]
macro_rules! json {
    ( $($tt:tt)* ) => { json_internal!($($tt)*) };
}

#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! json_internal {
    ({ $($tt:tt)+ }) => {{
        #[derive(Copy, Clone)]
        struct Map;
        struct MapState(usize);

        impl<'de> serde::de::Deserializer<'de> for Map {
            type Error = serde::de::value::Error;

            fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
            where
                V: serde::de::Visitor<'de>,
            {
                visitor.visit_map(MapState(0))
            }

            serde::forward_to_deserialize_any! {
                bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
                bytes byte_buf option unit unit_struct newtype_struct seq tuple
                tuple_struct map struct enum identifier ignored_any
            }
        }

        impl<'de> serde::de::MapAccess<'de> for MapState {
            type Error = serde::de::value::Error;

            fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
            where
                K: serde::de::DeserializeSeed<'de>,
            {
                let n = self.0;
                let res = json_internal!(@key (seed) n (0) () ($($tt)+));
                self.0 += 1;
                Ok(Some(res))
            }
            fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
            where
                V: serde::de::DeserializeSeed<'de>,
            {
                let n = self.0;
                let res = json_internal!(@skipkey (seed) n (0) () ($($tt)+));
                self.0 += 1;
                Ok(res)
            }
        }

        Map
    }};

    (@key ($f:expr) $n:ident ($i:expr) ($($key:tt)*) (: $($rest:tt)*)) => {
        if $n == $i {
            $f.deserialize(json_internal!($($key)*))?
        } else {
            json_internal!(@skipvalue ($f) $n ($i + 1) () ($($rest)*))
        }
    };
    (@key ($f:expr) $n:ident ($i:expr) ($($key:tt)*) ($tt:tt $($rest:tt)*)) => {
        json_internal!(@key ($f) $n ($i) ($($key)* $tt) ($($rest)*))
    };
    (@skipvalue ($f:expr) $n:ident ($i:expr) () (, $($rest:tt)+)) => {
        json_internal!(@key ($f) $n ($i + 1) () ($($rest)*))
    };
    (@skipvalue ($f:expr) $n:ident ($i:expr) () ($(,)?)) => {
        if $n == $i + 1 {
            return Ok(None)
        } else {
            return Err(<serde::de::value::Error as serde::de::Error>::custom("foobar"))
        }
    };
    (@skipvalue ($f:expr) $n:ident ($i:expr) () ($tt:tt $($rest:tt)*)) => {
        json_internal!(@skipvalue ($f) $n ($i) () ($($rest)*))
    };

    (@value ($f:expr) $n:ident ($i:expr) ($($value:tt)*) (, $($rest:tt)*)) => {
        if $n == $i {
            $f.deserialize(json_internal!($($value)*))?
        } else {
            json_internal!(@skipkey ($f) $n ($i + 1) () ($($rest)*))
        }
    };
    (@value ($f:expr) $n:ident ($i:expr) ($($value:tt)*) ($(,)?)) => {
        if $n == $i {
            $f.deserialize(json_internal!($($value)*))?
        } else {
            return Err(<serde::de::value::Error as serde::de::Error>::custom("foobar"))
        }
    };
    (@value ($f:expr) $n:ident ($i:expr) ($($value:tt)*) ($tt:tt $($rest:tt)*)) => {
        json_internal!(@value ($f) $n ($i) ($($value)* $tt) ($($rest)*))
    };
    (@skipkey ($f:expr) $n:ident ($i:expr) () (: $($rest:tt)+)) => {
        json_internal!(@value ($f) $n ($i + 1) () ($($rest)*))
    };
    (@skipkey ($f:expr) $n:ident ($i:expr) () ($tt:tt $($rest:tt)*)) => {
        json_internal!(@skipkey ($f) $n ($i) () ($($rest)*))
    };


    ($x:literal) => { $crate::Lit($x) };
}
