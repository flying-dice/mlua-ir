//! [`Shape`]: what a Rust type is to a script, as IR.
//!
//! A proc macro sees tokens and cannot tell what a type is. A trait can: `<T as Shape>::shape` resolves through
//! generics, aliases and other crates' types, so the macros only ever emit calls to it.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};

use crate::ir::Ty;
use crate::registry::Registry;

pub trait Shape {
    /// The type as a script sees it. A named type registers its definition in `reg` and returns a reference.
    fn shape(reg: &mut Registry) -> Ty;

    /// A parameter of this type may be left out by the caller.
    const OPTIONAL_PARAM: bool = false;

    /// A parameter of this type takes every remaining argument (only [`Variadic`]).
    const VARIADIC: bool = false;
}

/// The variadic last parameter: every remaining argument, `nil`s included (`f(sql, 1, nil)` passes two values,
/// which a list cannot carry). Declared `...T`. The macro recognises it by its name, so it must be written
/// `Variadic<T>` (or `mlua_ir::Variadic<T>`) as the last parameter.
pub struct Variadic<T>(pub Vec<T>);

impl<T: Shape> Shape for Variadic<T> {
    fn shape(reg: &mut Registry) -> Ty {
        T::shape(reg)
    }
    const OPTIONAL_PARAM: bool = true;
    const VARIADIC: bool = true;
}

impl Shape for () {
    fn shape(_: &mut Registry) -> Ty {
        Ty::Nil
    }
}

impl Shape for bool {
    fn shape(_: &mut Registry) -> Ty {
        Ty::Boolean
    }
}

impl Shape for String {
    fn shape(_: &mut Registry) -> Ty {
        Ty::String
    }
}

impl Shape for &'static str {
    fn shape(_: &mut Registry) -> Ty {
        Ty::String
    }
}

macro_rules! integer_shape {
    ($($t:ty),*) => {$(
        impl Shape for $t {
            fn shape(_: &mut Registry) -> Ty {
                // `MIN` of an unsigned type is 0, and every `MAX` fits a u64.
                #[allow(clippy::cast_lossless, clippy::cast_possible_wrap, clippy::cast_sign_loss)]
                Ty::Integer { min: <$t>::MIN as i64, max: <$t>::MAX as u64 }
            }
        }
    )*};
}
integer_shape!(i8, i16, i32, i64, u8, u16, u32, usize);

impl Shape for u64 {
    fn shape(_: &mut Registry) -> Ty {
        // A Lua number holds whole numbers exactly up to 2^53.
        Ty::Integer { min: 0, max: 1 << 53 }
    }
}

impl Shape for f64 {
    fn shape(_: &mut Registry) -> Ty {
        Ty::Number
    }
}

impl Shape for f32 {
    fn shape(_: &mut Registry) -> Ty {
        Ty::Number
    }
}

impl<T: Shape> Shape for Option<T> {
    fn shape(reg: &mut Registry) -> Ty {
        Ty::optional(T::shape(reg))
    }
    const OPTIONAL_PARAM: bool = true;
}

impl<T: Shape> Shape for Box<T> {
    fn shape(reg: &mut Registry) -> Ty {
        T::shape(reg)
    }
}

macro_rules! list_shape {
    ($($c:ident),*) => {$(
        impl<T: Shape> Shape for $c<T> {
            fn shape(reg: &mut Registry) -> Ty {
                Ty::List { item: Box::new(T::shape(reg)) }
            }
        }
    )*};
}
list_shape!(Vec, VecDeque, BTreeSet);

impl<T: Shape, S> Shape for HashSet<T, S> {
    fn shape(reg: &mut Registry) -> Ty {
        Ty::List { item: Box::new(T::shape(reg)) }
    }
}

impl<K: Shape, V: Shape, S> Shape for HashMap<K, V, S> {
    fn shape(reg: &mut Registry) -> Ty {
        Ty::Map { key: Box::new(K::shape(reg)), value: Box::new(V::shape(reg)) }
    }
}

impl<K: Shape, V: Shape> Shape for BTreeMap<K, V> {
    fn shape(reg: &mut Registry) -> Ty {
        Ty::Map { key: Box::new(K::shape(reg)), value: Box::new(V::shape(reg)) }
    }
}
