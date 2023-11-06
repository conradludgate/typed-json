#[macro_export(local_inner_macros)]
macro_rules! json {
    ( $($tt:tt)* ) => { json_internal!($($tt)*) };
}

// #[macro_export(local_inner_macros)]
// #[doc(hidden)]
// macro_rules! json_internal {
//     ({ $($tt:tt)+ }) => {{
//     }};

//     (@key ($f:expr) $n:ident ($i:expr) ($($key:tt)*) (: $($rest:tt)*)) => {
//         if $n == $i {
//             $f.deserialize(json_internal!($($key)*))?
//         } else {
//             json_internal!(@skipvalue ($f) $n ($i + 1) () ($($rest)*))
//         }
//     };
//     (@key ($f:expr) $n:ident ($i:expr) ($($key:tt)*) ($tt:tt $($rest:tt)*)) => {
//         json_internal!(@key ($f) $n ($i) ($($key)* $tt) ($($rest)*))
//     };
//     (@skipvalue ($f:expr) $n:ident ($i:expr) () (, $($rest:tt)+)) => {
//         json_internal!(@key ($f) $n ($i + 1) () ($($rest)*))
//     };
//     (@skipvalue ($f:expr) $n:ident ($i:expr) () ($(,)?)) => {
//         if $n == $i + 1 {
//             return Ok(None)
//         } else {
//             return Err(<serde::de::value::Error as serde::de::Error>::custom("foobar"))
//         }
//     };
//     (@skipvalue ($f:expr) $n:ident ($i:expr) () ($tt:tt $($rest:tt)*)) => {
//         json_internal!(@skipvalue ($f) $n ($i) () ($($rest)*))
//     };

//     (@value ($f:expr) $n:ident ($i:expr) ($($value:tt)*) (, $($rest:tt)*)) => {
//         if $n == $i {
//             $f.deserialize(json_internal!($($value)*))?
//         } else {
//             json_internal!(@skipkey ($f) $n ($i + 1) () ($($rest)*))
//         }
//     };
//     (@value ($f:expr) $n:ident ($i:expr) ($($value:tt)*) ($(,)?)) => {
//         if $n == $i {
//             $f.deserialize(json_internal!($($value)*))?
//         } else {
//             return Err(<serde::de::value::Error as serde::de::Error>::custom("foobar"))
//         }
//     };
//     (@value ($f:expr) $n:ident ($i:expr) ($($value:tt)*) ($tt:tt $($rest:tt)*)) => {
//         json_internal!(@value ($f) $n ($i) ($($value)* $tt) ($($rest)*))
//     };
//     (@skipkey ($f:expr) $n:ident ($i:expr) () (: $($rest:tt)+)) => {
//         json_internal!(@value ($f) $n ($i + 1) () ($($rest)*))
//     };
//     (@skipkey ($f:expr) $n:ident ($i:expr) () ($tt:tt $($rest:tt)*)) => {
//         json_internal!(@skipkey ($f) $n ($i) () ($($rest)*))
//     };

//     ($x:literal) => { $crate::Lit($x) };
// }

// Rocket relies on this because they export their own `json!` with a different
// doc comment than ours, and various Rust bugs prevent them from calling our
// `json!` from their `json!` so they call `json_internal!` directly. Check with
// @SergioBenitez before making breaking changes to this macro.
//
// Changes are fine as long as `json_internal!` does not call any new helper
// macros and can still be invoked as `json_internal!($($json)+)`.
#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! json_internal {
    //////////////////////////////////////////////////////////////////////////
    // TT muncher for parsing the inside of an array [...]. Produces a vec![...]
    // of the elements.
    //
    // Must be invoked as: json_internal!(@array [] $($tt)*)
    //////////////////////////////////////////////////////////////////////////

    // Done with trailing comma.
    (@array $seed:ident $n:ident [$($elems:expr,)*]) => {
        json_internal_vec![$seed $n (0) $($elems,)*]
    };

    // Done without trailing comma.
    (@array $seed:ident $n:ident [$($elems:expr),*]) => {
        json_internal_vec![$seed $n (0) $($elems),*]
    };

    // Next element is `null`.
    (@array $seed:ident $n:ident [$($elems:expr,)*] null $($rest:tt)*) => {
        json_internal!(@array $seed $n [$($elems,)* json_internal!(null)] $($rest)*)
    };

    // Next element is `true`.
    (@array $seed:ident $n:ident [$($elems:expr,)*] true $($rest:tt)*) => {
        json_internal!(@array $seed $n [$($elems,)* json_internal!(true)] $($rest)*)
    };

    // Next element is `false`.
    (@array $seed:ident $n:ident [$($elems:expr,)*] false $($rest:tt)*) => {
        json_internal!(@array $seed $n [$($elems,)* json_internal!(false)] $($rest)*)
    };

    // Next element is an array.
    (@array $seed:ident $n:ident [$($elems:expr,)*] [$($array:tt)*] $($rest:tt)*) => {
        json_internal!(@array $seed $n [$($elems,)* json_internal!([$($array)*])] $($rest)*)
    };

    // Next element is a map.
    (@array $seed:ident $n:ident [$($elems:expr,)*] {$($map:tt)*} $($rest:tt)*) => {
        json_internal!(@array $seed $n [$($elems,)* json_internal!({$($map)*})] $($rest)*)
    };

    // Next element is an expression followed by comma.
    (@array $seed:ident $n:ident [$($elems:expr,)*] $next:expr, $($rest:tt)*) => {
        json_internal!(@array $seed $n [$($elems,)* json_internal!($next),] $($rest)*)
    };

    // Last element is an expression with no trailing comma.
    (@array $seed:ident $n:ident [$($elems:expr,)*] $last:expr) => {
        json_internal!(@array $seed $n [$($elems,)* json_internal!($last)])
    };

    // Comma after the most recent element.
    (@array $seed:ident $n:ident [$($elems:expr),*] , $($rest:tt)*) => {
        json_internal!(@array $seed $n [$($elems,)*] $($rest)*)
    };

    // Unexpected token after most recent element.
    (@array $seed:ident $n:ident [$($elems:expr),*] $unexpected:tt $($rest:tt)*) => {
        json_unexpected!($unexpected)
    };

    ////

    // Done.
    (@arrayser $seed:ident []) => {};

    // Done with trailing comma.
    (@arrayser $seed:ident [$($elems:expr,)*]) => {
        json_internal![@arrayser $seed [$($elems),*]]
    };

    // Done without trailing comma.
    (@arrayser $seed:ident [$first:expr $(, $elems:expr)*]) => {
        serde::ser::SerializeSeq::serialize_element(&mut $seed, &$first)?;
        json_internal![@arrayser $seed [$($elems),*]]
    };

    // Next element is `null`.
    (@arrayser $seed:ident [$($elems:expr,)*] null $($rest:tt)*) => {
        json_internal!(@arrayser $seed [$($elems,)* json_internal!(null)] $($rest)*)
    };

    // Next element is `true`.
    (@arrayser $seed:ident [$($elems:expr,)*] true $($rest:tt)*) => {
        json_internal!(@arrayser $seed [$($elems,)* json_internal!(true)] $($rest)*)
    };

    // Next element is `false`.
    (@arrayser $seed:ident [$($elems:expr,)*] false $($rest:tt)*) => {
        json_internal!(@arrayser $seed [$($elems,)* json_internal!(false)] $($rest)*)
    };

    // Next element is an array.
    (@arrayser $seed:ident [$($elems:expr,)*] [$($array:tt)*] $($rest:tt)*) => {
        json_internal!(@arrayser $seed [$($elems,)* json_internal!([$($array)*])] $($rest)*)
    };

    // Next element is a map.
    (@arrayser $seed:ident [$($elems:expr,)*] {$($map:tt)*} $($rest:tt)*) => {
        json_internal!(@arrayser $seed [$($elems,)* json_internal!({$($map)*})] $($rest)*)
    };

    // Next element is an expression followed by comma.
    (@arrayser $seed:ident [$($elems:expr,)*] $next:expr, $($rest:tt)*) => {
        json_internal!(@arrayser $seed [$($elems,)* json_internal!($next),] $($rest)*)
    };

    // Last element is an expression with no trailing comma.
    (@arrayser $seed:ident [$($elems:expr,)*] $last:expr) => {
        json_internal!(@arrayser $seed [$($elems,)* json_internal!($last)])
    };

    // Comma after the most recent element.
    (@arrayser $seed:ident [$($elems:expr),*] , $($rest:tt)*) => {
        json_internal!(@arrayser $seed [$($elems,)*] $($rest)*)
    };

    // Unexpected token after most recent element.
    (@arrayser $seed:ident [$($elems:expr),*] $unexpected:tt $($rest:tt)*) => {
        json_unexpected!($unexpected)
    };
    //////////////////////////////////////////////////////////////////////////
    // TT muncher for parsing the inside of an object {...}. Each entry is
    // inserted into the given map variable.
    //
    // Must be invoked as: json_internal!(@object $map () ($($tt)*) ($($tt)*))
    //
    // We require two copies of the input tokens so that we can match on one
    // copy and trigger errors on the other copy.
    //////////////////////////////////////////////////////////////////////////

    // Done.
    (@objectvalue $seed:ident $n:ident ($i:expr)  () () ()) => {};

    // Insert the current entry followed by trailing comma.
    (@objectvalue $seed:ident $n:ident ($i:expr) [] ($value:expr) , $($rest:tt)*) => {
        if $n == $i {
            $seed.deserialize($value)?
        } else {
            json_internal!(@objectvalue $seed $n ($i + 2) () ($($rest)*) ($($rest)*))
        }
    };

    // Current entry followed by unexpected token.
    (@objectvalue $seed:ident $n:ident ($i:expr) [] ($value:expr) $unexpected:tt $($rest:tt)*) => {
        json_unexpected!($unexpected)
    };

    // Insert the last entry without trailing comma.
    (@objectvalue $seed:ident $n:ident ($i:expr) [] ($value:expr)) => {
        if $n == $i {
            $seed.deserialize($value)?
        } else {
            return Err(<serde::de::value::Error as serde::de::Error>::custom("foobar"))
        }
    };

    // Next value is `null`.
    (@objectvalue $seed:ident $n:ident ($i:expr) ($($key:tt)+) (: null $($rest:tt)*) $copy:tt) => {
        json_internal!(@objectvalue $seed $n ($i) [] (json_internal!(null)) $($rest)*)
    };

    // Next value is `true`.
    (@objectvalue $seed:ident $n:ident ($i:expr) ($($key:tt)+) (: true $($rest:tt)*) $copy:tt) => {
        json_internal!(@objectvalue $seed $n ($i) [] (json_internal!(true)) $($rest)*)
    };

    // Next value is `false`.
    (@objectvalue $seed:ident $n:ident ($i:expr) ($($key:tt)+) (: false $($rest:tt)*) $copy:tt) => {
        json_internal!(@objectvalue $seed $n ($i) [] (json_internal!(false)) $($rest)*)
    };

    // Next value is an array.
    (@objectvalue $seed:ident $n:ident ($i:expr) ($($key:tt)+) (: [$($array:tt)*] $($rest:tt)*) $copy:tt) => {
        json_internal!(@objectvalue $seed $n ($i) [] (json_internal!([$($array)*])) $($rest)*)
    };

    // Next value is a map.
    (@objectvalue $seed:ident $n:ident ($i:expr) ($($key:tt)+) (: {$($map:tt)*} $($rest:tt)*) $copy:tt) => {
        json_internal!(@objectvalue $seed $n ($i) [] (json_internal!({$($map)*})) $($rest)*)
    };

    // Next value is an expression followed by comma.
    (@objectvalue $seed:ident $n:ident ($i:expr) ($($key:tt)+) (: $value:expr , $($rest:tt)*) $copy:tt) => {
        json_internal!(@objectvalue $seed $n ($i) [] (json_internal!($value)) , $($rest)*)
    };

    // Last value is an expression with no trailing comma.
    (@objectvalue $seed:ident $n:ident ($i:expr) ($($key:tt)+) (: $value:expr) $copy:tt) => {
        json_internal!(@objectvalue $seed $n ($i) [] (json_internal!($value)))
    };

    // Missing value for last entry. Trigger a reasonable error message.
    (@objectvalue $seed:ident $n:ident ($i:expr) ($($key:tt)+) (:) $copy:tt) => {
        // "unexpected end of macro invocation"
        json_internal!()
    };

    // Missing colon and value for last entry. Trigger a reasonable error
    // message.
    (@objectvalue $seed:ident $n:ident ($i:expr) ($($key:tt)+) () $copy:tt) => {
        // "unexpected end of macro invocation"
        json_internal!()
    };

    // Misplaced colon. Trigger a reasonable error message.
    (@objectvalue $seed:ident $n:ident ($i:expr) () (: $($rest:tt)*) ($colon:tt $($copy:tt)*)) => {
        // Takes no arguments so "no rules expected the token `:`".
        json_unexpected!($colon)
    };

    // Found a comma inside a key. Trigger a reasonable error message.
    (@objectvalue $seed:ident $n:ident ($i:expr) ($($key:tt)*) (, $($rest:tt)*) ($comma:tt $($copy:tt)*)) => {
        // Takes no arguments so "no rules expected the token `,`".
        json_unexpected!($comma)
    };

    // Key is fully parenthesized. This avoids clippy double_parens false
    // positives because the parenthesization may be necessary here.
    (@objectvalue $seed:ident $n:ident ($i:expr) () (($key:expr) : $($rest:tt)*) $copy:tt) => {
        json_internal!(@objectvalue $seed $n ($i) ($key) (: $($rest)*) (: $($rest)*))
    };

    // Refuse to absorb colon token into key expression.
    (@objectvalue $seed:ident $n:ident ($i:expr) ($($key:tt)*) (: $($unexpected:tt)+) $copy:tt) => {
        json_expect_expr_comma!($($unexpected)+)
    };

    // Munch a token into the current key.
    (@objectvalue $seed:ident $n:ident ($i:expr) ($($key:tt)*) ($tt:tt $($rest:tt)*) $copy:tt) => {
        json_internal!(@objectvalue $seed $n ($i) ($($key)* $tt) ($($rest)*) ($($rest)*))
    };



    ////

    // Done.
    (@objectkey $seed:ident $n:ident ($i:expr)  () () ()) => {};

    // Insert the current entry followed by trailing comma.
    (@objectkey $seed:ident $n:ident ($i:expr) [$($key:tt)+] , $($rest:tt)*) => {
        if $n == $i {
            $seed.deserialize(json_internal!($($key)*))?
        } else {
            json_internal!(@objectkey $seed $n ($i + 2) () ($($rest)*) ($($rest)*))
        }
    };

    // Current entry followed by unexpected token.
    (@objectkey $seed:ident $n:ident ($i:expr) [$($key:tt)+] $unexpected:tt $($rest:tt)*) => {
        json_unexpected!($unexpected)
    };

    // Insert the last entry without trailing comma.
    (@objectkey $seed:ident $n:ident ($i:expr) [$($key:tt)+]) => {
        if $n == $i {
            $seed.deserialize(json_internal!($($key)*))?
        } else if $n == $i + 2 {
            return Ok(None)
        } else {
            return Err(<serde::de::value::Error as serde::de::Error>::custom("foobar"))
        }
    };

    // Next value is `null`.
    (@objectkey $seed:ident $n:ident ($i:expr) ($($key:tt)+) (: null $($rest:tt)*) $copy:tt) => {
        json_internal!(@objectkey $seed $n ($i) [$($key)+] $($rest)*)
    };

    // Next value is `true`.
    (@objectkey $seed:ident $n:ident ($i:expr) ($($key:tt)+) (: true $($rest:tt)*) $copy:tt) => {
        json_internal!(@objectkey $seed $n ($i) [$($key)+] $($rest)*)
    };

    // Next value is `false`.
    (@objectkey $seed:ident $n:ident ($i:expr) ($($key:tt)+) (: false $($rest:tt)*) $copy:tt) => {
        json_internal!(@objectkey $seed $n ($i) [$($key)+] $($rest)*)
    };

    // Next value is an array.
    (@objectkey $seed:ident $n:ident ($i:expr) ($($key:tt)+) (: [$($array:tt)*] $($rest:tt)*) $copy:tt) => {
        json_internal!(@objectkey $seed $n ($i) [$($key)+] $($rest)*)
    };

    // Next value is a map.
    (@objectkey $seed:ident $n:ident ($i:expr) ($($key:tt)+) (: {$($map:tt)*} $($rest:tt)*) $copy:tt) => {
        json_internal!(@objectkey $seed $n ($i) [$($key)+] $($rest)*)
    };

    // Next value is an expression followed by comma.
    (@objectkey $seed:ident $n:ident ($i:expr) ($($key:tt)+) (: $value:expr , $($rest:tt)*) $copy:tt) => {
        json_internal!(@objectkey $seed $n ($i) [$($key)+] , $($rest)*)
    };

    // Last value is an expression with no trailing comma.
    (@objectkey $seed:ident $n:ident ($i:expr) ($($key:tt)+) (: $value:expr) $copy:tt) => {
        json_internal!(@objectkey $seed $n ($i) [$($key)+])
    };

    // Missing value for last entry. Trigger a reasonable error message.
    (@objectkey $seed:ident $n:ident ($i:expr) ($($key:tt)+) (:) $copy:tt) => {
        // "unexpected end of macro invocation"
        json_internal!()
    };

    // Missing colon and value for last entry. Trigger a reasonable error
    // message.
    (@objectkey $seed:ident $n:ident ($i:expr) ($($key:tt)+) () $copy:tt) => {
        // "unexpected end of macro invocation"
        json_internal!()
    };

    // Misplaced colon. Trigger a reasonable error message.
    (@objectkey $seed:ident $n:ident ($i:expr) () (: $($rest:tt)*) ($colon:tt $($copy:tt)*)) => {
        // Takes no arguments so "no rules expected the token `:`".
        json_unexpected!($colon)
    };

    // Found a comma inside a key. Trigger a reasonable error message.
    (@objectkey $seed:ident $n:ident ($i:expr) ($($key:tt)*) (, $($rest:tt)*) ($comma:tt $($copy:tt)*)) => {
        // Takes no arguments so "no rules expected the token `,`".
        json_unexpected!($comma)
    };

    // Key is fully parenthesized. This avoids clippy double_parens false
    // positives because the parenthesization may be necessary here.
    (@objectkey $seed:ident $n:ident ($i:expr) () (($key:expr) : $($rest:tt)*) $copy:tt) => {
        json_internal!(@objectkey $seed $n ($i) ($key) (: $($rest)*) (: $($rest)*))
    };

    // Refuse to absorb colon token into key expression.
    (@objectkey $seed:ident $n:ident ($i:expr) ($($key:tt)*) (: $($unexpected:tt)+) $copy:tt) => {
        json_expect_expr_comma!($($unexpected)+)
    };

    // Munch a token into the current key.
    (@objectkey $seed:ident $n:ident ($i:expr) ($($key:tt)*) ($tt:tt $($rest:tt)*) $copy:tt) => {
        json_internal!(@objectkey $seed $n ($i) ($($key)* $tt) ($($rest)*) ($($rest)*))
    };

    ////

    // Done.
    (@objectser $seed:ident () () ()) => {};

    // Insert the current entry followed by trailing comma.
    (@objectser $seed:ident [$($key:tt)+] ($value:expr) , $($rest:tt)*) => {
        serde::ser::SerializeMap::serialize_key(&mut $seed, &json_internal!($($key)*))?;
        serde::ser::SerializeMap::serialize_value(&mut $seed, &$value)?;
    };

    // Current entry followed by unexpected token.
    (@objectser $seed:ident [$($key:tt)+] ($value:expr) $unexpected:tt $($rest:tt)*) => {
        json_unexpected!($unexpected)
    };

    // Insert the last entry without trailing comma.
    (@objectser $seed:ident [$($key:tt)+] ($value:expr)) => {
        serde::ser::SerializeMap::serialize_key(&mut $seed, &json_internal!($($key)*))?;
        serde::ser::SerializeMap::serialize_value(&mut $seed, &$value)?;
    };

    // Next value is `null`.
    (@objectser $seed:ident ($($key:tt)+) (: null $($rest:tt)*) $copy:tt) => {
        json_internal!(@objectser $seed [$($key)+] (json_internal!(null)) $($rest)*)
    };

    // Next value is `true`.
    (@objectser $seed:ident ($($key:tt)+) (: true $($rest:tt)*) $copy:tt) => {
        json_internal!(@objectser $seed [$($key)+] (json_internal!(true)) $($rest)*)
    };

    // Next value is `false`.
    (@objectser $seed:ident ($($key:tt)+) (: false $($rest:tt)*) $copy:tt) => {
        json_internal!(@objectser $seed [$($key)+] (json_internal!(false)) $($rest)*)
    };

    // Next value is an array.
    (@objectser $seed:ident ($($key:tt)+) (: [$($array:tt)*] $($rest:tt)*) $copy:tt) => {
        json_internal!(@objectser $seed [$($key)+] (json_internal!([$($array)*])) $($rest)*)
    };

    // Next value is a map.
    (@objectser $seed:ident ($($key:tt)+) (: {$($map:tt)*} $($rest:tt)*) $copy:tt) => {
        json_internal!(@objectser $seed [$($key)+] (json_internal!({$($map)*})) $($rest)*)
    };

    // Next value is an expression followed by comma.
    (@objectser $seed:ident ($($key:tt)+) (: $value:expr , $($rest:tt)*) $copy:tt) => {
        json_internal!(@objectser $seed [$($key)+] (json_internal!($value)) , $($rest)*)
    };

    // Last value is an expression with no trailing comma.
    (@objectser $seed:ident ($($key:tt)+) (: $value:expr) $copy:tt) => {
        json_internal!(@objectser $seed [$($key)+] (json_internal!($value)))
    };

    // Missing value for last entry. Trigger a reasonable error message.
    (@objectser $seed:ident ($($key:tt)+) (:) $copy:tt) => {
        // "unexpected end of macro invocation"
        json_internal!()
    };

    // Missing colon and value for last entry. Trigger a reasonable error
    // message.
    (@objectser $seed:ident ($($key:tt)+) () $copy:tt) => {
        // "unexpected end of macro invocation"
        json_internal!()
    };

    // Misplaced colon. Trigger a reasonable error message.
    (@objectser $seed:ident () (: $($rest:tt)*) ($colon:tt $($copy:tt)*)) => {
        // Takes no arguments so "no rules expected the token `:`".
        json_unexpected!($colon)
    };

    // Found a comma inside a key. Trigger a reasonable error message.
    (@objectser $seed:ident ($($key:tt)*) (, $($rest:tt)*) ($comma:tt $($copy:tt)*)) => {
        // Takes no arguments so "no rules expected the token `,`".
        json_unexpected!($comma)
    };

    // Key is fully parenthesized. This avoids clippy double_parens false
    // positives because the parenthesization may be necessary here.
    (@objectser $seed:ident () (($key:expr) : $($rest:tt)*) $copy:tt) => {
        json_internal!(@objectser $seed ($key) (: $($rest)*) (: $($rest)*))
    };

    // Refuse to absorb colon token into key expression.
    (@objectser $seed:ident ($($key:tt)*) (: $($unexpected:tt)+) $copy:tt) => {
        json_expect_expr_comma!($($unexpected)+)
    };

    // Munch a token into the current key.
    (@objectser $seed:ident ($($key:tt)*) ($tt:tt $($rest:tt)*) $copy:tt) => {
        json_internal!(@objectser $seed ($($key)* $tt) ($($rest)*) ($($rest)*))
    };


    //////////////////////////////////////////////////////////////////////////
    // The main implementation.
    //
    // Must be invoked as: json_internal!($($json)+)
    //////////////////////////////////////////////////////////////////////////

    (null) => {
        $crate::Null
    };

    (true) => {
        $crate::Bool(true)
    };

    (false) => {
        $crate::Bool(false)
    };

    ([]) => {
        $crate::EmptyList
    };

    ([ $($tt:tt)+ ]) => {{

        #[derive(Copy, Clone)]
        struct List;
        struct ListState(usize);

        impl<'de> serde::de::Deserializer<'de> for List {
            type Error = serde::de::value::Error;

            fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
            where
                V: serde::de::Visitor<'de>,
            {
                visitor.visit_seq(ListState(0))
            }

            serde::forward_to_deserialize_any! {
                bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
                bytes byte_buf option unit unit_struct newtype_struct seq tuple
                tuple_struct map struct enum identifier ignored_any
            }
        }

        impl<'de> serde::de::SeqAccess<'de> for ListState {
            type Error = serde::de::value::Error;

            fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
            where
                T: serde::de::DeserializeSeed<'de>,
            {
                let n = self.0;
                let res = json_internal!(@array seed n [] $($tt)+);
                self.0 += 1;
                Ok(Some(res))
            }
        }

        impl serde::ser::Serialize for List {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let mut seq = serializer.serialize_seq(Some(json_internal_length!(@array [] $($tt)+)))?;
                json_internal!(@arrayser seq [] $($tt)+);
                serde::ser::SerializeSeq::end(seq)
            }
        }

        List
    }};

    ({}) => {
        $crate::EmptyMap
    };

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
                let res = json_internal!(@objectkey seed n (0) () ($($tt)+) ($($tt)+));
                self.0 += 1;
                Ok(Some(res))
            }
            fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
            where
                V: serde::de::DeserializeSeed<'de>,
            {
                let n = self.0;
                let res = json_internal!(@objectvalue seed n (1) () ($($tt)+) ($($tt)+));
                self.0 += 1;
                Ok(res)
            }
        }

        impl serde::ser::Serialize for Map {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let mut seq = serializer.serialize_map(Some(json_internal_length!(@object () ($($tt)+) ($($tt)+))))?;
                json_internal!(@objectser seq () ($($tt)+) ($($tt)+));
                serde::ser::SerializeMap::end(seq)
            }
        }

        Map
    }};

    // Any Serialize type: numbers, strings, struct literals, variables etc.
    // Must be below every other rule.
    ($other:expr) => {
        $crate::Lit($other)
    };
}

// The json_internal macro above cannot invoke vec directly because it uses
// local_inner_macros. A vec invocation there would resolve to $crate::vec.
// Instead invoke vec here outside of local_inner_macros.
#[macro_export]
#[doc(hidden)]
macro_rules! json_internal_vec {
    ($seed:ident $n:ident ($i:expr) $first:expr $(, $rest:expr)*) => {
        if $n == $i {
            $seed.deserialize($first)?
        } else {
            json_internal_vec!($seed $n ($i + 1) $($rest),*)
        }
    };
    ($seed:ident $n:ident ($i:expr)) => {
        if $n == $i {
            return Ok(None);
        } else {
            return Err(<serde::de::value::Error as serde::de::Error>::custom("foobar"))
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! json_unexpected {
    () => {};
}

#[macro_export]
#[doc(hidden)]
macro_rules! json_expect_expr_comma {
    ($e:expr , $($tt:tt)*) => {};
}

#[macro_export]
#[doc(hidden)]
macro_rules! json_internal_length {
    // Done.
    (@array []) => {
        0
    };
    // Done with trailing comma.
    (@array [$($elems:expr,)*]) => {
        json_internal_length![@array [$($elems),*]]
    };

    // Done without trailing comma.
    (@array [$first:expr $(, $rest:expr)*]) => {
        1 + json_internal_length![@array [$($rest),*]]
    };

    // Next element is `null`.
    (@array [$($elems:expr,)*] null $($rest:tt)*) => {
        json_internal_length!(@array [$($elems,)* json_internal!(null)] $($rest)*)
    };

    // Next element is `true`.
    (@array [$($elems:expr,)*] true $($rest:tt)*) => {
        json_internal_length!(@array [$($elems,)* json_internal!(true)] $($rest)*)
    };

    // Next element is `false`.
    (@array [$($elems:expr,)*] false $($rest:tt)*) => {
        json_internal_length!(@array [$($elems,)* json_internal!(false)] $($rest)*)
    };

    // Next element is an array.
    (@array [$($elems:expr,)*] [$($array:tt)*] $($rest:tt)*) => {
        json_internal_length!(@array [$($elems,)* json_internal!([$($array)*])] $($rest)*)
    };

    // Next element is a map.
    (@array [$($elems:expr,)*] {$($map:tt)*} $($rest:tt)*) => {
        json_internal_length!(@array [$($elems,)* json_internal!({$($map)*})] $($rest)*)
    };

    // Next element is an expression followed by comma.
    (@array [$($elems:expr,)*] $next:expr, $($rest:tt)*) => {
        json_internal_length!(@array [$($elems,)* json_internal!($next),] $($rest)*)
    };

    // Last element is an expression with no trailing comma.
    (@array [$($elems:expr,)*] $last:expr) => {
        json_internal_length!(@array [$($elems,)* json_internal!($last)])
    };

    // Comma after the most recent element.
    (@array [$($elems:expr),*] , $($rest:tt)*) => {
        json_internal_length!(@array [$($elems,)*] $($rest)*)
    };

    // Unexpected token after most recent element.
    (@array [$($elems:expr),*] $unexpected:tt $($rest:tt)*) => {
        json_unexpected!($unexpected)
    };

    // Done.
    (@object () () ()) => {};

    // Insert the current entry followed by trailing comma.
    (@object [] () , $($rest:tt)*) => {
        1 + json_internal_length!(@object () ($($rest)*) ($($rest)*))
    };

    // Current entry followed by unexpected token.
    (@object [] () $unexpected:tt $($rest:tt)*) => {
        json_unexpected!($unexpected)
    };

    // Insert the last entry without trailing comma.
    (@object [] ()) => {
        1
    };

    // Next value is `null`.
    (@object ($($key:tt)+) (: null $($rest:tt)*) $copy:tt) => {
        json_internal_length!(@object [] () $($rest)*)
    };

    // Next value is `true`.
    (@object ($($key:tt)+) (: true $($rest:tt)*) $copy:tt) => {
        json_internal_length!(@object [] () $($rest)*)
    };

    // Next value is `false`.
    (@object ($($key:tt)+) (: false $($rest:tt)*) $copy:tt) => {
        json_internal_length!(@object [] () $($rest)*)
    };

    // Next value is an array.
    (@object ($($key:tt)+) (: [$($array:tt)*] $($rest:tt)*) $copy:tt) => {
        json_internal_length!(@object [] () $($rest)*)
    };

    // Next value is a map.
    (@object ($($key:tt)+) (: {$($map:tt)*} $($rest:tt)*) $copy:tt) => {
        json_internal_length!(@object [] () $($rest)*)
    };

    // Next value is an expression followed by comma.
    (@object ($($key:tt)+) (: $value:expr , $($rest:tt)*) $copy:tt) => {
        json_internal_length!(@object [] () , $($rest)*)
    };

    // Last value is an expression with no trailing comma.
    (@object ($($key:tt)+) (: $value:expr) $copy:tt) => {
        json_internal_length!(@object [] ())
    };

    // Missing value for last entry. Trigger a reasonable error message.
    (@object ($($key:tt)+) (:) $copy:tt) => {
        // "unexpected end of macro invocation"
        json_internal_length!()
    };

    // Missing colon and value for last entry. Trigger a reasonable error
    // message.
    (@object ($($key:tt)+) () $copy:tt) => {
        // "unexpected end of macro invocation"
        json_internal_length!()
    };

    // Misplaced colon. Trigger a reasonable error message.
    (@object () (: $($rest:tt)*) ($colon:tt $($copy:tt)*)) => {
        // Takes no arguments so "no rules expected the token `:`".
        json_unexpected!($colon)
    };

    // Found a comma inside a key. Trigger a reasonable error message.
    (@object ($($key:tt)*) (, $($rest:tt)*) ($comma:tt $($copy:tt)*)) => {
        // Takes no arguments so "no rules expected the token `,`".
        json_unexpected!($comma)
    };

    // Key is fully parenthesized. This avoids clippy double_parens false
    // positives because the parenthesization may be necessary here.
    (@object () (($key:expr) : $($rest:tt)*) $copy:tt) => {
        json_internal_length!(@object ($key) (: $($rest)*) (: $($rest)*))
    };

    // Refuse to absorb colon token into key expression.
    (@object ($($key:tt)*) (: $($unexpected:tt)+) $copy:tt) => {
        json_expect_expr_comma!($($unexpected)+)
    };

    // Munch a token into the current key.
    (@object ($($key:tt)*) ($tt:tt $($rest:tt)*) $copy:tt) => {
        json_internal_length!(@object ($($key)* $tt) ($($rest)*) ($($rest)*))
    };

}
