/*!
 * Behold, the smoking remains of some ravaged Rust code, the result of an
 * invocation gone awry, for Yandros had been awakened from their slumber.
 *
 * The Intelligence Dampening Sphere, thinking it could overtake Rust and its
 * constraints, had engaged in a devastating battle.
 *
 * But the Crab was no easy creature to subdue, and no matter how many
 * cursed poorman specialization stabs were taken against it, the Crab remained,
 * unscathed.
 *
 * Unwilling to admit defeat, the challenger, desperate, started chanting an
 * ancient incantation from the forbidden times, unwittingly dooming us all:
 *
 * ```rs
 * #[allow(unstable_features)]
 * #[feature(specialization)]
 * ```
 *
 * The Crab immediately unleashed an ear-piercing shriek of pain, after which
 * there was no sound: complete and utter silence.
 *
 * ---
 *
 * Tread carefully among these paths, adventurer, and forgo any hope for sane
 * Rust code.
 */
#![cfg_attr(rustfmt, rustfmt::skip)]

use ::serde::{
    de::Deserializer as De,
    ser::Serialize as Ser,
};

#[allow(unused)]
macro_rules! emit {( $($it:item)* ) => ( $($it)* )}

#[inline]
pub
fn zst_expr<'de, R, E>(
    f: impl FnOnce() -> R,
) -> impl ::serde::ser::Serialize + ::serde::de::Deserializer<'de, Error = E>
where
    R : Ser + De<'de, Error = E>,
    E : ::serde::de::Error,
{
    trait Helper<'de, R : Ser + De<'de>> : FnOnce() -> R {
        type Ret : Ser + De<'de, Error = R::Error>;
        fn into_serdable(self) -> Self::Ret;
    }

macro_rules! with_default {( $($default:ident)? ) => (
    impl<'de, F : FnOnce() -> R, R : Ser + De<'de>> Helper<'de, R> for F {
        $($default)?
        type Ret = R;

        #[inline]
        $($default)?
        fn into_serdable(self) -> Self::Ret {
            let r: R = self();
            unsafe {
                ::core::mem::transmute_copy(&::core::mem::ManuallyDrop::new(r))
            }
        }
    }
)}
    #[cfg(feature = "specialization")]
    with_default!(default);
    #[cfg(not(feature = "specialization"))]
    with_default!();

    #[cfg(feature = "specialization")]
    impl<'de, F : Fn() -> R, R : Ser + De<'de>> Helper<'de, R> for F {
        type Ret = SerdeFn<F>;

        #[inline]
        fn into_serdable(self) -> Self::Ret {
            SerdeFn(self)
        }
    }

    Helper::<R>::into_serdable(f)
}

#[doc(hidden)] /** Not part of the public API */ #[macro_export]
macro_rules! ඞzst_expr {(
    $e:expr $(,)?
) => (
    // match || $e { f => {
    //     let discriminator = $crate::ඞ::Discriminator::NEW;
    //     if false {
    //         discriminator.set(f);
    //         loop {}
    //     }
    //     #[allow(unused)]
    //     use $crate::ඞ::DefaultImpl as _;
    //     $crate::ඞ::Discriminator::__ඞwrap(discriminator, f)
    // }}
    $crate::ඞ::zst_expr(|| $e)
)}
pub use ඞzst_expr as zst_expr;

#[cfg(feature = "specialization")] emit! {
    pub
    struct SerdeFn<F>(F);

    impl<F: Fn() -> T, T : ::serde::ser::Serialize>
        ::serde::ser::Serialize
    for
        SerdeFn<F>
    {
        #[inline]
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S : ::serde::ser::Serializer,
        {
            self.0().serialize(serializer)
        }
    }

    impl<'de, F: Fn() -> T, T : ::serde::de::Deserializer<'de>>
        ::serde::de::Deserializer<'de>
    for
        SerdeFn<F>
    {
        type Error = T::Error;

        ::serde::forward_to_deserialize_any! {
            bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
            bytes byte_buf option unit unit_struct newtype_struct seq tuple
            tuple_struct map struct enum identifier ignored_any
        }

        fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, T::Error>
        where
            V : ::serde::de::Visitor<'de>,
        {
            self.0().deserialize_any(visitor)
        }
    }
}

#[test]
fn conrad_pls()
where
    'static :,
{}

#[test]
fn conrad_pls_dont_do_it()
{
    struct ScopeGuard<F: FnMut()>(F);
    impl<F : FnMut()> Drop for ScopeGuard<F> {
        fn drop(&mut self)
        {}
    }

    let mut slot = None;
    return match &drop(()) { temporary => {
        slot.replace(ScopeGuard(move || _ = (temporary, )));
    }}
}
