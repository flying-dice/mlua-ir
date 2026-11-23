//! [`IntoReturn`]: what a Rust function's return type is to a script.
//!
//! | Rust return | A script gets |
//! | --- | --- |
//! | `()` | nothing |
//! | `T` | `T` |
//! | `(A, B)` | `A, B` |
//! | `Result<T, E>` (`E: LuaError`) | `(T?, E?)`: an expected failure is a returned value |
//! | `Result<T, Raise<E>>` | `T`; an expected failure is raised as `E` |

use mlua::{Lua, MultiValue, Value};

use crate::convert::IntoLuaRet;
use crate::error::{LuaError, Raise, raised};
use crate::ir::{Returns, Ty};
use crate::registry::Registry;
use crate::shape::Shape;

pub trait IntoReturn {
    fn returns(reg: &mut Registry) -> Returns;

    /// What the call raises when it fails expectedly.
    fn raises(_reg: &mut Registry) -> Option<Ty> {
        None
    }

    fn into_return(self, lua: &Lua, op: &'static str) -> mlua::Result<MultiValue>;
}

impl IntoReturn for () {
    fn returns(_: &mut Registry) -> Returns {
        Returns::None
    }
    fn into_return(self, _: &Lua, _: &'static str) -> mlua::Result<MultiValue> {
        Ok(MultiValue::new())
    }
}

impl<T: Shape + IntoLuaRet> IntoReturn for T {
    fn returns(reg: &mut Registry) -> Returns {
        Returns::One { ty: T::shape(reg) }
    }
    fn into_return(self, lua: &Lua, op: &'static str) -> mlua::Result<MultiValue> {
        Ok(MultiValue::from_vec(vec![self.into_lua_ret(lua, op)?]))
    }
}

macro_rules! tuple_return {
    ($($t:ident),+) => {
        impl<$($t: Shape + IntoLuaRet),+> IntoReturn for ($($t,)+) {
            fn returns(reg: &mut Registry) -> Returns {
                Returns::Many { tys: vec![$($t::shape(reg)),+] }
            }
            #[allow(non_snake_case)]
            fn into_return(self, lua: &Lua, op: &'static str) -> mlua::Result<MultiValue> {
                let ($($t,)+) = self;
                Ok(MultiValue::from_vec(vec![$($t.into_lua_ret(lua, op)?),+]))
            }
        }
    };
}
tuple_return!(A, B);
tuple_return!(A, B, C);

impl<T: Shape + IntoLuaRet, E: LuaError> IntoReturn for Result<T, E> {
    fn returns(reg: &mut Registry) -> Returns {
        Returns::Fallible { ok: T::shape(reg), err: E::shape(reg) }
    }
    fn into_return(self, lua: &Lua, op: &'static str) -> mlua::Result<MultiValue> {
        Ok(match self {
            Ok(v) => MultiValue::from_vec(vec![v.into_lua_ret(lua, op)?, Value::Nil]),
            Err(e) => MultiValue::from_vec(vec![Value::Nil, e.into_lua_ret(lua, op)?]),
        })
    }
}

/// A call that succeeds with nothing to say returns `true` (so `assert(call())` reads naturally).
impl<E: LuaError> IntoReturn for Result<(), E> {
    fn returns(reg: &mut Registry) -> Returns {
        Returns::Fallible { ok: Ty::Boolean, err: E::shape(reg) }
    }
    fn into_return(self, lua: &Lua, op: &'static str) -> mlua::Result<MultiValue> {
        Ok(match self {
            Ok(()) => MultiValue::from_vec(vec![Value::Boolean(true), Value::Nil]),
            Err(e) => MultiValue::from_vec(vec![Value::Nil, e.into_lua_ret(lua, op)?]),
        })
    }
}

impl<T: IntoReturn, E: LuaError> IntoReturn for Result<T, Raise<E>> {
    fn returns(reg: &mut Registry) -> Returns {
        T::returns(reg)
    }
    fn raises(reg: &mut Registry) -> Option<Ty> {
        Some(E::shape(reg))
    }
    fn into_return(self, lua: &Lua, op: &'static str) -> mlua::Result<MultiValue> {
        match self {
            Ok(v) => v.into_return(lua, op),
            Err(Raise(e)) => Err(raised(&e, op)),
        }
    }
}

/// An error from Lua itself (a callback that raised) passes through: it is raised again, unchanged.
impl<T: IntoReturn> IntoReturn for Result<T, mlua::Error> {
    fn returns(reg: &mut Registry) -> Returns {
        T::returns(reg)
    }
    fn raises(reg: &mut Registry) -> Option<Ty> {
        T::raises(reg)
    }
    fn into_return(self, lua: &Lua, op: &'static str) -> mlua::Result<MultiValue> {
        self.and_then(|v| v.into_return(lua, op))
    }
}
