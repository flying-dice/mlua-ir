//! Every enum a serde type reaches, with all of its variant names.
//!
//! serde-reflection stops exploring behind an `Option` once it knows the field's format, so an enum behind
//! `Option` (every enum in bollard's models) is found with one variant. serde's derived `Deserialize` hands every
//! enum's full list of (renamed) variant names to `deserialize_enum`, so a deserializer that walks the whole type,
//! always taking `Some`, one item of each list and every field of each struct, learns them all.

use std::collections::BTreeMap;

use serde::de::{self, DeserializeSeed, Deserializer, IntoDeserializer, Visitor};

pub type Enums = BTreeMap<String, Vec<String>>;

#[derive(Debug)]
struct Stop(String);

impl std::fmt::Display for Stop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
impl std::error::Error for Stop {}
impl de::Error for Stop {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Stop(msg.to_string())
    }
}

/// The enums `T` reaches. Best effort: a part of the type the walk cannot get through is skipped.
pub fn enums<'de, T: de::Deserialize<'de>>() -> Enums {
    let mut found = Enums::new();
    let mut stack = Vec::new();
    if let Err(e) = T::deserialize(Walker { found: &mut found, stack: &mut stack, depth: 0 })
        && std::env::var_os("MLUA_IR_DEBUG").is_some()
    {
        eprintln!("harvesting {}: {e}", std::any::type_name::<T>());
    }
    found
}

/// A backstop only: recursion is stopped by `stack` (a struct is not entered again inside itself).
const MAX_DEPTH: usize = 256;

struct Walker<'a> {
    found: &'a mut Enums,
    /// The structs being walked, outermost first.
    stack: &'a mut Vec<&'static str>,
    depth: usize,
}

impl Walker<'_> {
    fn deeper(&mut self) -> Result<Walker<'_>, Stop> {
        if self.depth >= MAX_DEPTH {
            return Err(Stop("too deep".into()));
        }
        Ok(Walker { found: self.found, stack: self.stack, depth: self.depth + 1 })
    }
}

macro_rules! leaf {
    ($($method:ident => $visit:ident($($value:expr)?)),* $(,)?) => {$(
        fn $method<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Stop> {
            visitor.$visit($($value)?)
        }
    )*};
}

impl<'de> Deserializer<'de> for Walker<'_> {
    type Error = Stop;

    leaf! {
        deserialize_bool => visit_bool(false),
        deserialize_i8 => visit_i8(0), deserialize_i16 => visit_i16(0), deserialize_i32 => visit_i32(0),
        deserialize_i64 => visit_i64(0), deserialize_u8 => visit_u8(0), deserialize_u16 => visit_u16(0),
        deserialize_u32 => visit_u32(0), deserialize_u64 => visit_u64(0), deserialize_f32 => visit_f32(0.0),
        deserialize_f64 => visit_f64(0.0), deserialize_char => visit_char('a'),
        deserialize_str => visit_str(""), deserialize_string => visit_string(String::new()),
        deserialize_bytes => visit_bytes(&[]), deserialize_byte_buf => visit_byte_buf(Vec::new()),
        deserialize_unit => visit_unit(), deserialize_identifier => visit_str(""),
        deserialize_ignored_any => visit_unit(), deserialize_any => visit_unit(),
    }

    fn deserialize_option<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Stop> {
        visitor.visit_some(self.deeper()?)
    }

    fn deserialize_unit_struct<V: Visitor<'de>>(self, _: &'static str, visitor: V) -> Result<V::Value, Stop> {
        visitor.visit_unit()
    }

    fn deserialize_newtype_struct<V: Visitor<'de>>(mut self, _: &'static str, visitor: V) -> Result<V::Value, Stop> {
        visitor.visit_newtype_struct(self.deeper()?)
    }

    fn deserialize_seq<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Stop> {
        let inner = self.deeper()?;
        visitor.visit_seq(One { item: Some(inner) })
    }

    fn deserialize_tuple<V: Visitor<'de>>(self, _: usize, visitor: V) -> Result<V::Value, Stop> {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V: Visitor<'de>>(self, _: &'static str, _: usize, visitor: V) -> Result<V::Value, Stop> {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Stop> {
        let inner = self.deeper()?;
        visitor.visit_map(Entry { walker: inner, done: false })
    }

    fn deserialize_struct<V: Visitor<'de>>(
        mut self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Stop> {
        if self.stack.contains(&name) {
            return Err(Stop(format!("{name} contains itself")));
        }
        self.stack.push(name);
        let walked = {
            let inner = self.deeper()?;
            visitor.visit_map(Fields { names: fields.to_vec(), next: 0, walker: inner })
        };
        self.stack.pop();
        walked
    }

    fn deserialize_enum<V: Visitor<'de>>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Stop> {
        self.found.insert(name.to_owned(), variants.iter().map(|v| (*v).to_owned()).collect());
        let first = variants.first().ok_or_else(|| Stop("an enum without variants".into()))?;
        visitor.visit_enum(first.into_deserializer())
    }
}

struct One<'a> {
    item: Option<Walker<'a>>,
}

impl<'de> de::SeqAccess<'de> for One<'_> {
    type Error = Stop;
    fn next_element_seed<T: DeserializeSeed<'de>>(&mut self, seed: T) -> Result<Option<T::Value>, Stop> {
        match self.item.take() {
            Some(walker) => seed.deserialize(walker).map(Some),
            None => Ok(None),
        }
    }
}

struct Fields<'a> {
    names: Vec<&'static str>,
    next: usize,
    walker: Walker<'a>,
}

impl<'de> de::MapAccess<'de> for Fields<'_> {
    type Error = Stop;
    fn next_key_seed<K: DeserializeSeed<'de>>(&mut self, seed: K) -> Result<Option<K::Value>, Stop> {
        let Some(name) = self.names.get(self.next) else { return Ok(None) };
        self.next += 1;
        seed.deserialize(de::value::StrDeserializer::<Stop>::new(name)).map(Some)
    }
    fn next_value_seed<V: DeserializeSeed<'de>>(&mut self, seed: V) -> Result<V::Value, Stop> {
        seed.deserialize(self.walker.deeper()?)
    }
}

/// One entry of a map, its key walked like any value (bollard has maps keyed by `()`).
struct Entry<'a> {
    walker: Walker<'a>,
    done: bool,
}

impl<'de> de::MapAccess<'de> for Entry<'_> {
    type Error = Stop;
    fn next_key_seed<K: DeserializeSeed<'de>>(&mut self, seed: K) -> Result<Option<K::Value>, Stop> {
        if self.done {
            return Ok(None);
        }
        self.done = true;
        seed.deserialize(self.walker.deeper()?).map(Some)
    }
    fn next_value_seed<V: DeserializeSeed<'de>>(&mut self, seed: V) -> Result<V::Value, Stop> {
        seed.deserialize(self.walker.deeper()?)
    }
}
