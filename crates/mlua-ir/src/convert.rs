//! Lua values in and out, strictly: a value that is not what the shape says is refused with the reason, never
//! coerced (`"12"` is not a number, `1.5` is not a `u32`, a table with holes is not a list).

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::hash::{BuildHasher, Hash};

use mlua::serde::SerializeOptions;
use mlua::{IntoLua, Lua, LuaSerdeExt, Value};
use serde::Serialize;
use serde::de::DeserializeOwned;

/// Read from a script's value. The error is the reason alone; the caller adds where (`argument #2 'width'`).
pub trait FromLuaArg: Sized {
    fn from_lua_arg(value: Value, lua: &Lua) -> Result<Self, String>;
}

/// Given to a script. `op` names the call (`std:fs.read`), for the values that carry it (errors, streams).
pub trait IntoLuaRet {
    fn into_lua_ret(self, lua: &Lua, op: &'static str) -> mlua::Result<Value>;
}

/// A type that is plain data to a script and converts through serde (what `#[derive(Shape)]` marks).
pub trait SerdeShape {}

impl<T: SerdeShape + DeserializeOwned> FromLuaArg for T {
    fn from_lua_arg(value: Value, lua: &Lua) -> Result<Self, String> {
        from_serde(value, lua)
    }
}

impl<T: SerdeShape + Serialize> IntoLuaRet for T {
    fn into_lua_ret(self, lua: &Lua, _: &'static str) -> mlua::Result<Value> {
        to_serde(&self, lua)
    }
}

/// A value read through serde, with mlua's prefix taken off the reason.
pub fn from_serde<T: DeserializeOwned>(value: Value, lua: &Lua) -> Result<T, String> {
    lua.from_value(value).map_err(|e| match e {
        mlua::Error::DeserializeError(reason) => reason,
        other => other.to_string(),
    })
}

/// A value written through serde: `None` is `nil` (not mlua's `null` sentinel), and lists are plain tables.
pub fn to_serde<T: Serialize + ?Sized>(value: &T, lua: &Lua) -> mlua::Result<Value> {
    lua.to_value_with(
        value,
        SerializeOptions::new()
            .serialize_none_to_null(false)
            .serialize_unit_to_null(false)
            .set_array_metatable(false),
    )
}

/// `expected a string, got number`.
pub fn expected(what: &str, value: &Value) -> String {
    format!("expected {what}, got {}", describe(value))
}

fn describe(value: &Value) -> String {
    match value {
        Value::String(s) => match s.to_str() {
            Ok(text) if text.len() <= 32 => format!("\"{}\"", &*text),
            _ => "string".to_owned(),
        },
        Value::Number(n) => format!("{n}"),
        Value::Integer(n) => format!("{n}"),
        other => other.type_name().to_owned(),
    }
}

impl FromLuaArg for () {
    fn from_lua_arg(_: Value, _: &Lua) -> Result<Self, String> {
        Ok(())
    }
}

impl FromLuaArg for bool {
    fn from_lua_arg(value: Value, _: &Lua) -> Result<Self, String> {
        match value {
            Value::Boolean(b) => Ok(b),
            other => Err(expected("a boolean", &other)),
        }
    }
}

impl IntoLuaRet for bool {
    fn into_lua_ret(self, _: &Lua, _: &'static str) -> mlua::Result<Value> {
        Ok(Value::Boolean(self))
    }
}

impl FromLuaArg for String {
    fn from_lua_arg(value: Value, _: &Lua) -> Result<Self, String> {
        match value {
            Value::String(s) => s.to_str().map(|t| t.to_owned()).map_err(|_| "expected UTF-8 text".to_owned()),
            other => Err(expected("a string", &other)),
        }
    }
}

impl IntoLuaRet for String {
    fn into_lua_ret(self, lua: &Lua, _: &'static str) -> mlua::Result<Value> {
        self.into_lua(lua)
    }
}

impl IntoLuaRet for &'static str {
    fn into_lua_ret(self, lua: &Lua, _: &'static str) -> mlua::Result<Value> {
        self.into_lua(lua)
    }
}

/// A whole number, from either of mlua's number variants.
fn whole(value: &Value) -> Option<f64> {
    match *value {
        #[allow(clippy::cast_precision_loss)]
        Value::Integer(i) => Some(i as f64),
        Value::Number(n) if n.is_finite() && n.fract() == 0.0 => Some(n),
        _ => None,
    }
}

macro_rules! integer_convert {
    ($($t:ty),*) => {$(
        impl FromLuaArg for $t {
            fn from_lua_arg(value: Value, _: &Lua) -> Result<Self, String> {
                let what = || format!("a whole number from {} to {}", <$t>::MIN, <$t>::MAX);
                let n = whole(&value).ok_or_else(|| expected(&what(), &value))?;
                #[allow(clippy::cast_possible_truncation)]
                let i = n as i128;
                <$t>::try_from(i).map_err(|_| expected(&what(), &value))
            }
        }
        impl IntoLuaRet for $t {
            fn into_lua_ret(self, lua: &Lua, _: &'static str) -> mlua::Result<Value> {
                self.into_lua(lua)
            }
        }
    )*};
}
integer_convert!(i8, i16, i32, i64, u8, u16, u32, u64, usize);

impl FromLuaArg for f64 {
    fn from_lua_arg(value: Value, _: &Lua) -> Result<Self, String> {
        match value {
            Value::Number(n) => Ok(n),
            #[allow(clippy::cast_precision_loss)]
            Value::Integer(i) => Ok(i as f64),
            other => Err(expected("a number", &other)),
        }
    }
}

impl IntoLuaRet for f64 {
    fn into_lua_ret(self, _: &Lua, _: &'static str) -> mlua::Result<Value> {
        Ok(Value::Number(self))
    }
}

impl<T: FromLuaArg> FromLuaArg for Option<T> {
    fn from_lua_arg(value: Value, lua: &Lua) -> Result<Self, String> {
        match value {
            Value::Nil => Ok(None),
            value => T::from_lua_arg(value, lua).map(Some),
        }
    }
}

impl<T: IntoLuaRet> IntoLuaRet for Option<T> {
    fn into_lua_ret(self, lua: &Lua, op: &'static str) -> mlua::Result<Value> {
        match self {
            Some(v) => v.into_lua_ret(lua, op),
            None => Ok(Value::Nil),
        }
    }
}

/// The items of a table that is a list: keys exactly `1..=n`, read in order.
fn list_items<T: FromLuaArg>(value: Value, lua: &Lua) -> Result<Vec<T>, String> {
    let Value::Table(table) = value else {
        return Err(expected("a list (a table)", &value));
    };
    let mut entries: Vec<(i64, Value)> = Vec::new();
    for pair in table.pairs::<Value, Value>() {
        let (key, item) = pair.map_err(|e| e.to_string())?;
        let index = whole(&key)
            .filter(|n| *n >= 1.0)
            .ok_or_else(|| format!("a list has only the keys 1, 2, 3, ...; found the key {}", describe(&key)))?;
        #[allow(clippy::cast_possible_truncation)]
        entries.push((index as i64, item));
    }
    entries.sort_by_key(|(index, _)| *index);
    let mut items = Vec::with_capacity(entries.len());
    for (expected_index, (index, item)) in (1..).zip(entries) {
        if index != expected_index {
            return Err(format!("a list has no holes; item {expected_index} is missing"));
        }
        items.push(T::from_lua_arg(item, lua).map_err(|e| format!("item {index}: {e}"))?);
    }
    Ok(items)
}

fn list_value<T: IntoLuaRet>(items: impl IntoIterator<Item = T>, lua: &Lua, op: &'static str) -> mlua::Result<Value> {
    let table = lua.create_table()?;
    for (index, item) in (1..).zip(items) {
        table.raw_set(index, item.into_lua_ret(lua, op)?)?;
    }
    Ok(Value::Table(table))
}

impl<T: FromLuaArg> FromLuaArg for Vec<T> {
    fn from_lua_arg(value: Value, lua: &Lua) -> Result<Self, String> {
        list_items(value, lua)
    }
}

impl<T: IntoLuaRet> IntoLuaRet for Vec<T> {
    fn into_lua_ret(self, lua: &Lua, op: &'static str) -> mlua::Result<Value> {
        list_value(self, lua, op)
    }
}

impl<T: FromLuaArg> FromLuaArg for VecDeque<T> {
    fn from_lua_arg(value: Value, lua: &Lua) -> Result<Self, String> {
        list_items(value, lua).map(VecDeque::from)
    }
}

impl<T: IntoLuaRet> IntoLuaRet for VecDeque<T> {
    fn into_lua_ret(self, lua: &Lua, op: &'static str) -> mlua::Result<Value> {
        list_value(self, lua, op)
    }
}

impl<T: FromLuaArg + Ord> FromLuaArg for BTreeSet<T> {
    fn from_lua_arg(value: Value, lua: &Lua) -> Result<Self, String> {
        list_items(value, lua).map(BTreeSet::from_iter)
    }
}

impl<T: IntoLuaRet> IntoLuaRet for BTreeSet<T> {
    fn into_lua_ret(self, lua: &Lua, op: &'static str) -> mlua::Result<Value> {
        list_value(self, lua, op)
    }
}

impl<T: FromLuaArg + Eq + Hash, S: BuildHasher + Default> FromLuaArg for HashSet<T, S> {
    fn from_lua_arg(value: Value, lua: &Lua) -> Result<Self, String> {
        list_items(value, lua).map(HashSet::from_iter)
    }
}

impl<T: IntoLuaRet, S> IntoLuaRet for HashSet<T, S> {
    fn into_lua_ret(self, lua: &Lua, op: &'static str) -> mlua::Result<Value> {
        list_value(self, lua, op)
    }
}

fn map_entries<K: FromLuaArg, V: FromLuaArg>(value: Value, lua: &Lua) -> Result<Vec<(K, V)>, String> {
    let Value::Table(table) = value else {
        return Err(expected("a map (a table)", &value));
    };
    let mut entries = Vec::new();
    for pair in table.pairs::<Value, Value>() {
        let (key, item) = pair.map_err(|e| e.to_string())?;
        let shown = describe(&key);
        let key = K::from_lua_arg(key, lua).map_err(|e| format!("key {shown}: {e}"))?;
        let item = V::from_lua_arg(item, lua).map_err(|e| format!("[{shown}]: {e}"))?;
        entries.push((key, item));
    }
    Ok(entries)
}

fn map_value<K: IntoLuaRet, V: IntoLuaRet>(
    entries: impl IntoIterator<Item = (K, V)>,
    lua: &Lua,
    op: &'static str,
) -> mlua::Result<Value> {
    let table = lua.create_table()?;
    for (key, item) in entries {
        table.raw_set(key.into_lua_ret(lua, op)?, item.into_lua_ret(lua, op)?)?;
    }
    Ok(Value::Table(table))
}

impl<K: FromLuaArg + Eq + Hash, V: FromLuaArg, S: BuildHasher + Default> FromLuaArg for HashMap<K, V, S> {
    fn from_lua_arg(value: Value, lua: &Lua) -> Result<Self, String> {
        map_entries(value, lua).map(HashMap::from_iter)
    }
}

impl<K: IntoLuaRet, V: IntoLuaRet, S> IntoLuaRet for HashMap<K, V, S> {
    fn into_lua_ret(self, lua: &Lua, op: &'static str) -> mlua::Result<Value> {
        map_value(self, lua, op)
    }
}

impl<K: FromLuaArg + Ord, V: FromLuaArg> FromLuaArg for BTreeMap<K, V> {
    fn from_lua_arg(value: Value, lua: &Lua) -> Result<Self, String> {
        map_entries(value, lua).map(BTreeMap::from_iter)
    }
}

impl<K: IntoLuaRet, V: IntoLuaRet> IntoLuaRet for BTreeMap<K, V> {
    fn into_lua_ret(self, lua: &Lua, op: &'static str) -> mlua::Result<Value> {
        map_value(self, lua, op)
    }
}
