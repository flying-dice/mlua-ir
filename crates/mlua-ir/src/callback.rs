//! [`Callback`]: a Lua function the Rust side calls, typed by a Rust function-pointer type.
//!
//! `Callback<fn(left: String, right: String) -> bool>` declares `(left: string, right: string) -> boolean`: the
//! parameter names are read from the signature by the module macro, the types through [`Shape`].

use std::marker::PhantomData;

use mlua::{Function, Lua, MultiValue, Value, WeakLua};

use crate::convert::{FromLuaArg, IntoLuaRet, expected};
use crate::ir::{Param, Returns, Ty};
use crate::registry::Registry;
use crate::shape::Shape;

pub struct Callback<F> {
    func: Function,
    lua: WeakLua,
    _signature: PhantomData<F>,
}

impl<F> Callback<F> {
    /// The script's function, for a host that runs it itself (as a coroutine, on a scheduler).
    pub fn function(&self) -> &Function {
        &self.func
    }

    pub fn into_function(self) -> Function {
        self.func
    }
}

fn returns_of<R: Shape>(reg: &mut Registry) -> Returns {
    match R::shape(reg) {
        Ty::Nil => Returns::None,
        ty => Returns::One { ty },
    }
}

/// Renames the parameters of the first function type in `ty` (the macro passes the names it read).
pub fn name_params(ty: &mut Ty, names: &[&str]) {
    match ty {
        Ty::Function { params, .. } => {
            for (param, name) in params.iter_mut().zip(names) {
                param.name = (*name).to_owned();
            }
        }
        Ty::Optional { inner } => name_params(inner, names),
        _ => {}
    }
}

macro_rules! callback_arity {
    ($($a:ident),*) => {
        impl<$($a: Shape,)* R: Shape> Shape for Callback<fn($($a),*) -> R> {
            fn shape(reg: &mut Registry) -> Ty {
                #[allow(unused_mut)]
                let mut index = 0;
                let params = vec![$({
                    index += 1;
                    Param { name: format!("arg{index}"), ty: $a::shape(reg), optional: false }
                }),*];
                let _ = index;
                Ty::Function { params, returns: Box::new(returns_of::<R>(reg)) }
            }
        }

        impl<$($a,)* R> FromLuaArg for Callback<fn($($a),*) -> R> {
            fn from_lua_arg(value: Value, lua: &Lua) -> Result<Self, String> {
                match value {
                    Value::Function(func) => Ok(Callback { func, lua: lua.weak(), _signature: PhantomData }),
                    other => Err(expected("a function", &other)),
                }
            }
        }

        impl<$($a: IntoLuaRet,)* R: FromLuaArg> Callback<fn($($a),*) -> R> {
            /// Calls the script's function. What it raises comes back as the error.
            #[allow(non_snake_case, clippy::too_many_arguments)]
            pub fn call(&self, $($a: $a),*) -> mlua::Result<R> {
                let lua = self.lua.upgrade();
                let args = MultiValue::from_vec(vec![$($a.into_lua_ret(&lua, "callback")?),*]);
                let value: Value = self.func.call(args)?;
                R::from_lua_arg(value, &lua).map_err(|e| mlua::Error::runtime(format!("the callback returned {e}")))
            }
        }
    };
}
callback_arity!();
callback_arity!(A);
callback_arity!(A, B);
callback_arity!(A, B, C);
