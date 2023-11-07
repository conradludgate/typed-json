/// Construct an [`impl serde::Serialize`](serde::Serialize) from a JSON literal.
///
/// ```
/// # use typed_json::json;
/// #
/// let value = json!({
///     "code": 200,
///     "success": true,
///     "payload": {
///         "features": [
///             "serde",
///             "json"
///         ],
///         "homepage": null
///     }
/// });
/// ```
///
/// Variables or expressions can be interpolated into the JSON literal. Any type
/// interpolated into an array element or object value must implement Serde's
/// `Serialize` trait, while any type interpolated into a object key must
/// implement `Into<String>`. If the `Serialize` implementation of the
/// interpolated type decides to fail, or if the interpolated type contains a
/// map with non-string keys, the `json!` macro will panic.
///
/// ```
/// # use typed_json::json;
/// #
/// let code = 200;
/// let features = vec!["typed", "json"];
///
/// let value = json!({
///     "code": code,
///     "success": code == 200,
///     "payload": {
///         features[0]: features[1]
///     }
/// });
/// ```
///
/// Trailing commas are allowed inside both arrays and objects.
///
/// ```
/// # use typed_json::json;
/// #
/// let value = json!([
///     "notice",
///     "the",
///     "trailing",
///     "comma -->",
/// ]);
/// ```
#[macro_export(local_inner_macros)]
macro_rules! json {
    // Hide distracting implementation details from the generated rustdoc.
    ($($json:tt)+) => {
        $crate::serialize(json_internal!($($json)+))
    };
}

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
    (@array [$($elems:expr,)*]) => {
        json_internal_vec![$($elems,)*]
    };

    // Done without trailing comma.
    (@array [$($elems:expr),*]) => {
        json_internal_vec![$($elems),*]
    };

    // Next element is `null`.
    (@array [$($elems:expr,)*] null $($rest:tt)*) => {
        json_internal!(@array [$($elems,)* json_internal!(null)] $($rest)*)
    };

    // Next element is `true`.
    (@array [$($elems:expr,)*] true $($rest:tt)*) => {
        json_internal!(@array [$($elems,)* json_internal!(true)] $($rest)*)
    };

    // Next element is `false`.
    (@array [$($elems:expr,)*] false $($rest:tt)*) => {
        json_internal!(@array [$($elems,)* json_internal!(false)] $($rest)*)
    };

    // Next element is an array.
    (@array [$($elems:expr,)*] [$($array:tt)*] $($rest:tt)*) => {
        json_internal!(@array [$($elems,)* json_internal!([$($array)*])] $($rest)*)
    };

    // Next element is a map.
    (@array [$($elems:expr,)*] {$($map:tt)*} $($rest:tt)*) => {
        json_internal!(@array [$($elems,)* json_internal!({$($map)*})] $($rest)*)
    };

    // Next element is an expression followed by comma.
    (@array [$($elems:expr,)*] $next:expr, $($rest:tt)*) => {
        json_internal!(@array [$($elems,)* json_internal!($next),] $($rest)*)
    };

    // Last element is an expression with no trailing comma.
    (@array [$($elems:expr,)*] $last:expr) => {
        json_internal!(@array [$($elems,)* json_internal!($last)])
    };

    // Comma after the most recent element.
    (@array [$($elems:expr),*] , $($rest:tt)*) => {
        json_internal!(@array [$($elems,)*] $($rest)*)
    };

    // Unexpected token after most recent element.
    (@array [$($elems:expr),*] $unexpected:tt $($rest:tt)*) => {
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
    (@object () () ()) => { () };

    // Insert the current entry followed by trailing comma.
    (@object [$($key:tt)+] ($value:expr) , $($rest:tt)*) => {
        $crate::KVList {
            first: ::core::option::Option::Some($crate::KV::Pair(json_internal!($($key)*), $value)),
            second: json_internal!(@object () ($($rest)*) ($($rest)*)),
        }
    };

    // Current entry followed by unexpected token.
    (@object [$($key:tt)+] ($value:expr) $unexpected:tt $($rest:tt)*) => {
        json_unexpected!($unexpected)
    };

    // Insert the last entry without trailing comma.
    (@object [$($key:tt)+] ($value:expr)) => {
        $crate::KVList {
            first: ::core::option::Option::Some($crate::KV::Pair(json_internal!($($key)*), $value)),
            second: (),
        }
    };

    // Next value is `null`.
    (@object ($($key:tt)+) (: null $($rest:tt)*) $copy:tt) => {
        json_internal!(@object [$($key)+] (json_internal!(null)) $($rest)*)
    };

    // Next value is `true`.
    (@object ($($key:tt)+) (: true $($rest:tt)*) $copy:tt) => {
        json_internal!(@object [$($key)+] (json_internal!(true)) $($rest)*)
    };

    // Next value is `false`.
    (@object ($($key:tt)+) (: false $($rest:tt)*) $copy:tt) => {
        json_internal!(@object [$($key)+] (json_internal!(false)) $($rest)*)
    };

    // Next value is an array.
    (@object ($($key:tt)+) (: [$($array:tt)*] $($rest:tt)*) $copy:tt) => {
        json_internal!(@object [$($key)+] (json_internal!([$($array)*])) $($rest)*)
    };

    // Next value is a map.
    (@object ($($key:tt)+) (: {$($map:tt)*} $($rest:tt)*) $copy:tt) => {
        json_internal!(@object [$($key)+] (json_internal!({$($map)*})) $($rest)*)
    };

    // Next value is an expression followed by comma.
    (@object ($($key:tt)+) (: $value:expr , $($rest:tt)*) $copy:tt) => {
        json_internal!(@object [$($key)+] (json_internal!($value)) , $($rest)*)
    };

    // Last value is an expression with no trailing comma.
    (@object ($($key:tt)+) (: $value:expr) $copy:tt) => {
        json_internal!(@object [$($key)+] (json_internal!($value)))
    };

    // Missing value for last entry. Trigger a reasonable error message.
    (@object ($($key:tt)+) (:) $copy:tt) => {
        // "unexpected end of macro invocation"
        json_internal!()
    };

    // Missing colon and value for last entry. Trigger a reasonable error
    // message.
    (@object ($($key:tt)+) () $copy:tt) => {
        // "unexpected end of macro invocation"
        json_internal!()
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
        json_internal!(@object ($key) (: $($rest)*) (: $($rest)*))
    };

    // Refuse to absorb colon token into key expression.
    (@object ($($key:tt)*) (: $($unexpected:tt)+) $copy:tt) => {
        json_expect_expr_comma!($($unexpected)+)
    };

    // Munch a token into the current key.
    (@object ($($key:tt)*) ($tt:tt $($rest:tt)*) $copy:tt) => {
        json_internal!(@object ($($key)* $tt) ($($rest)*) ($($rest)*))
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
        $crate::Expr(true)
    };

    (false) => {
        $crate::Expr(false)
    };

    ([]) => {
        $crate::List(())
    };

    ([ $($tt:tt)+ ]) => {
        $crate::List(json_internal!(@array [] $($tt)+))
    };

    ({}) => {
        $crate::Map(())
    };

    ({ $($tt:tt)+ }) => {
        $crate::Map(json_internal!(@object () ($($tt)+) ($($tt)+)))
    };

    // Any Serialize type: numbers, strings, struct literals, variables etc.
    // Must be below every other rule.
    ($other:expr) => {
        $crate::Expr($other)
    };
}

// The json_internal macro above cannot invoke vec directly because it uses
// local_inner_macros. A vec invocation there would resolve to $crate::vec.
// Instead invoke vec here outside of local_inner_macros.
#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! json_internal_vec {
    ($first:expr $(, $rest:expr)* $(,)?) => {
        $crate::List1 {
            first: ::core::option::Option::Some($first),
            second:  json_internal_vec!($($rest),*),
        }
    };
    () => {
        ()
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
