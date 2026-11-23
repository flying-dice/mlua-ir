//! Checks a script's value against a type's declared shape before serde reads it.
//!
//! serde alone is lenient where Lua is ambiguous: mlua reads a keyed table as an empty list, so
//! `flags = { verbose = {...} }` would become zero flags. Every serde-read argument (records, options, other crates'
//! types) is checked here first, so what the declarations refuse is refused at run time too, with the path to it.

use std::any::TypeId;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use mlua::{Lua, Value};
use serde::de::DeserializeOwned;

use crate::convert::{expected, from_serde};
use crate::ir::{Field, Ty, TypeBody, TypeDef};
use crate::registry::Registry;
use crate::shape::Shape;

type Shaped = Rc<(Ty, HashMap<String, TypeDef>)>;

thread_local! {
    static SHAPES: RefCell<HashMap<TypeId, Shaped>> = RefCell::new(HashMap::new());
}

/// `T`'s shape and the named types it reaches, read once per thread. `module` names the module that types
/// without one of their own join (another crate's types).
pub(crate) fn shape_of<T: Shape + 'static>(module: Option<&str>) -> Shaped {
    SHAPES.with(|cache| {
        cache
            .borrow_mut()
            .entry(TypeId::of::<T>())
            .or_insert_with(|| {
                let mut reg = Registry::new();
                if let Some(module) = module {
                    reg.enter_module(module);
                }
                let ty = T::shape(&mut reg);
                let types = reg.into_types().into_iter().map(|t| (t.name.clone(), t)).collect();
                Rc::new((ty, types))
            })
            .clone()
    })
}

/// Reads `value` as `T` through serde, after checking it against `T`'s shape.
pub fn from_checked<T: Shape + DeserializeOwned + 'static>(value: Value, lua: &Lua) -> Result<T, String> {
    let shaped = shape_of::<T>(None);
    check(&value, &shaped.0, &shaped.1, "")?;
    from_serde(value, lua)
}

fn at(path: &str) -> String {
    if path.is_empty() { String::new() } else { format!("{path}: ") }
}

fn whole_index(key: &Value) -> Option<i64> {
    match *key {
        Value::Integer(i) if i >= 1 => Some(i),
        #[allow(clippy::cast_possible_truncation)]
        Value::Number(n) if n >= 1.0 && n.fract() == 0.0 && n.is_finite() => Some(n as i64),
        _ => None,
    }
}

fn fields_of(value: &Value, fields: &[Field], types: &HashMap<String, TypeDef>, path: &str, skip: Option<&str>) -> Result<(), String> {
    let Value::Table(table) = value else { return Err(format!("{}{}", at(path), expected("a table", value))) };
    for pair in table.pairs::<Value, Value>() {
        let (key, entry) = pair.map_err(|e| e.to_string())?;
        let key = key.to_string().unwrap_or_default();
        if skip == Some(key.as_str()) {
            continue;
        }
        let Some(field) = fields.iter().find(|f| f.name == key) else {
            let names: Vec<&str> = fields.iter().map(|f| f.name.as_str()).collect();
            return Err(format!("{}unknown field `{key}`, expected one of {}", at(path), names.join(", ")));
        };
        let inner = if path.is_empty() { key.clone() } else { format!("{path}.{key}") };
        check(&entry, &field.ty, types, &inner)?;
    }
    Ok(())
}

/// Checks `value` against `ty`. Only what serde would get wrong or explain badly is checked here: lists that are
/// not sequences, unknown keys, strings outside an enum, a tag no variant has. Scalars are left to serde.
pub(crate) fn check(value: &Value, ty: &Ty, types: &HashMap<String, TypeDef>, path: &str) -> Result<(), String> {
    match (ty, value) {
        (Ty::Optional { .. }, Value::Nil) => Ok(()),
        (Ty::Optional { inner }, value) => check(value, inner, types, path),
        (Ty::List { item }, Value::Table(table)) => {
            let mut entries = Vec::new();
            for pair in table.pairs::<Value, Value>() {
                let (key, entry) = pair.map_err(|e| e.to_string())?;
                let Some(index) = whole_index(&key) else {
                    let shown = key.to_string().unwrap_or_else(|_| key.type_name().to_owned());
                    return Err(format!("{}a list has only the keys 1, 2, 3, ...; found the key `{shown}`", at(path)));
                };
                entries.push((index, entry));
            }
            entries.sort_by_key(|(index, _)| *index);
            for (expected_index, (index, entry)) in (1..).zip(&entries) {
                if *index != expected_index {
                    return Err(format!("{}a list has no holes; item {expected_index} is missing", at(path)));
                }
                check(entry, item, types, &format!("{path}[{index}]"))?;
            }
            Ok(())
        }
        (Ty::Map { value: item, .. }, Value::Table(table)) => {
            for pair in table.pairs::<Value, Value>() {
                let (key, entry) = pair.map_err(|e| e.to_string())?;
                let shown = key.to_string().unwrap_or_default();
                check(&entry, item, types, &format!("{path}[\"{shown}\"]"))?;
            }
            Ok(())
        }
        (Ty::Union { members }, value) => {
            if members.iter().any(|m| check(value, m, types, path).is_ok()) {
                Ok(())
            } else {
                members.iter().map(|m| check(value, m, types, path)).find_map(Result::err).map_or(Ok(()), Err)
            }
        }
        (Ty::Named { name }, value) => match types.get(name).map(|t| &t.body) {
            Some(TypeBody::Record { fields } | TypeBody::Options { fields }) => fields_of(value, fields, types, path, None),
            Some(TypeBody::TaggedUnion { tag, variants }) => {
                let Value::Table(table) = value else { return Err(format!("{}{}", at(path), expected("a table", value))) };
                let tag_value: Option<String> = table.raw_get(tag.as_str()).ok();
                let names: Vec<String> = variants.iter().map(|v| format!("\"{}\"", v.tag_value)).collect();
                match variants.iter().find(|v| Some(&v.tag_value) == tag_value.as_ref()) {
                    Some(variant) => fields_of(value, &variant.fields, types, path, Some(tag)),
                    None => Err(format!("{}`{tag}` must be one of {}", at(path), names.join(", "))),
                }
            }
            Some(TypeBody::StringEnum { variants }) => match value {
                Value::String(s) if variants.iter().any(|v| s.to_str().is_ok_and(|t| *t == v.value)) => Ok(()),
                other => {
                    let names: Vec<String> = variants.iter().map(|v| format!("\"{}\"", v.value)).collect();
                    Err(format!("{}{}", at(path), expected(&format!("one of {}", names.join(", ")), other)))
                }
            },
            Some(TypeBody::Alias { ty }) => check(value, ty, types, path),
            _ => Ok(()),
        },
        _ => Ok(()),
    }
}
