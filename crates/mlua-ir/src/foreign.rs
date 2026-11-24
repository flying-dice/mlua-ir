//! [`Foreign`]: another crate's serde types, typed without writing their shape again.
//!
//! The orphan rule stops this crate implementing [`Shape`] for `bollard::models::ContainerSummary`, and a proc
//! macro cannot see into another crate. What can is serde: [`serde_reflection`] traces the type's `Deserialize`
//! and reports every struct, field and enum variant under the names serde uses, which are the names a script
//! sees (`Id`, `Names`, `State`). The traced types join the module that reaches them (`vnd_bollard_ContainerSummary`).
//!
//! The same shape checks what a script passes in: a key the type does not have is refused, as it is for this
//! crate's own records (serde alone would drop it silently).

use std::any::TypeId;
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};

use mlua::{Lua, Value};
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_reflection::{ContainerFormat, Format, FormatHolder, Named, Samples, Tracer, TracerConfig, VariantFormat};

use crate::convert::{FromLuaArg, IntoLuaRet, from_serde, to_serde};
use crate::ir::{EnumVariant, Field, Ty, TypeBody};
use crate::registry::{Definition, Registry};
use crate::shape::Shape;

/// A value of another crate's type, given to and read from a script as plain data.
pub struct Foreign<T>(pub T);

/// What tracing found: the top-level format and every container it reaches.
struct Traced {
    format: Format,
    containers: BTreeMap<String, ContainerFormat>,
    /// Every enum's full list of variant names (serde-reflection alone finds one per enum behind an `Option`).
    enums: crate::harvest::Enums,
}

thread_local! {
    static TRACED: RefCell<HashMap<TypeId, std::rc::Rc<Traced>>> = RefCell::new(HashMap::new());
}

/// The strings a trace reads wherever a type asks for one, tried in order. Most types take any string, and some
/// enums that parse their own text accept only `""` (bollard's `EMPTY` variants); a type whose `Deserialize`
/// validates a date (bollard's models with its `time` feature) refuses `""`, so a valid date-time is the fallback.
pub(crate) const SAMPLE_TEXTS: [&str; 2] = ["", "1970-01-01T00:00:00Z"];

fn config(sample: &'static str) -> TracerConfig {
    TracerConfig::default().default_borrowed_str_value(sample).default_string_value(sample.to_owned())
}

/// One complete trace of `T` with `sample` as every string.
fn trace_with<T: DeserializeOwned>(sample: &'static str) -> serde_reflection::Result<(Format, BTreeMap<String, ContainerFormat>)> {
    let samples = Samples::new();
    // serde-reflection walks one variant of a nested enum per pass and marks the enum incomplete; clearing the
    // marks and walking again finds the rest. A first pass learns the enums' names.
    let mut probe = Tracer::new(config(sample));
    probe.trace_type_once::<T>(&samples)?;
    let enums: Vec<String> = probe
        .registry_unchecked()
        .into_iter()
        .filter(|(_, c)| matches!(c, ContainerFormat::Enum(_)))
        .map(|(name, _)| name)
        .collect();
    let mut tracer = Tracer::new(config(sample));
    let mut format = loop {
        let (format, _) = tracer.trace_type_once::<T>(&samples)?;
        let mut incomplete = false;
        for name in &enums {
            incomplete |= tracer.check_incomplete_enum(name).is_some();
        }
        if !incomplete {
            break format;
        }
    };
    format.normalize().ok();
    Ok((format, tracer.registry()?))
}

fn traced<T: DeserializeOwned + 'static>() -> std::rc::Rc<Traced> {
    TRACED.with(|cache| {
        cache
            .borrow_mut()
            .entry(TypeId::of::<T>())
            .or_insert_with(|| {
                let mut last = None;
                for sample in SAMPLE_TEXTS {
                    match trace_with::<T>(sample) {
                        Ok((format, containers)) => {
                            return std::rc::Rc::new(Traced { format, containers, enums: crate::harvest::enums::<T>() });
                        }
                        Err(e) => last = Some(e),
                    }
                }
                panic!("tracing {}: {}", std::any::type_name::<T>(), last.map(|e| e.to_string()).unwrap_or_default())
            })
            .clone()
    })
}

fn ty_of(format: &Format, module: Option<&str>) -> Ty {
    match format {
        Format::Variable(v) => match v.borrow().as_ref() {
            Some(inner) => ty_of(inner, module),
            None => Ty::Unknown { reason: "a format serde-reflection could not resolve".into() },
        },
        Format::TypeName(name) => Ty::named(Registry::qualify(module, name)),
        Format::Unit => Ty::Nil,
        Format::Bool => Ty::Boolean,
        Format::I8 => Ty::Integer { min: i8::MIN.into(), max: 127 },
        Format::I16 => Ty::Integer { min: i16::MIN.into(), max: 32_767 },
        Format::I32 => Ty::Integer { min: i32::MIN.into(), max: 2_147_483_647 },
        Format::I64 | Format::I128 => Ty::Integer { min: i64::MIN, max: i64::MAX.unsigned_abs() },
        Format::U8 => Ty::Integer { min: 0, max: 255 },
        Format::U16 => Ty::Integer { min: 0, max: 65_535 },
        Format::U32 => Ty::Integer { min: 0, max: 4_294_967_295 },
        Format::U64 | Format::U128 => Ty::Integer { min: 0, max: 1 << 53 },
        Format::F32 | Format::F64 => Ty::Number,
        Format::Char | Format::Str | Format::Bytes => Ty::String,
        Format::Option(inner) => Ty::optional(ty_of(inner, module)),
        Format::Seq(item) | Format::TupleArray { content: item, .. } => {
            Ty::List { item: Box::new(ty_of(item, module)) }
        }
        // bollard writes Docker's always-empty objects (`ExposedPorts = { ["80/tcp"] = {} }`) as `HashMap<(), ()>`.
        Format::Map { key, value } if matches!(**key, Format::Unit) && matches!(**value, Format::Unit) => {
            Ty::Map { key: Box::new(Ty::String), value: Box::new(Ty::Nil) }
        }
        Format::Map { key, value } => {
            Ty::Map { key: Box::new(ty_of(key, module)), value: Box::new(ty_of(value, module)) }
        }
        Format::Tuple(items) => Ty::List { item: Box::new(Ty::Union { members: items.iter().map(|f| ty_of(f, module)).collect() }) },
    }
}

fn fields_of(fields: &[Named<Format>], module: Option<&str>) -> Vec<Field> {
    fields
        .iter()
        .map(|f| Field { name: f.name.clone(), doc: None, ty: ty_of(&f.value, module), default: None, readonly: false })
        .collect()
}

fn body_of(name: &str, container: &ContainerFormat, enums: &crate::harvest::Enums, module: Option<&str>) -> TypeBody {
    match container {
        ContainerFormat::Enum(variants)
            if variants.values().all(|v| matches!(v.value, VariantFormat::Unit)) && enums.contains_key(name) =>
        {
            TypeBody::StringEnum {
                variants: enums[name].iter().map(|v| EnumVariant { value: v.clone(), doc: None }).collect(),
            }
        }
        ContainerFormat::Struct(fields) => TypeBody::Record { fields: fields_of(fields, module) },
        ContainerFormat::NewTypeStruct(inner) => TypeBody::Alias { ty: ty_of(inner, module) },
        ContainerFormat::UnitStruct => TypeBody::Alias { ty: Ty::Nil },
        ContainerFormat::TupleStruct(items) => TypeBody::Alias {
            ty: Ty::List { item: Box::new(Ty::Union { members: items.iter().map(|f| ty_of(f, module)).collect() }) },
        },
        ContainerFormat::Enum(variants) if variants.values().all(|v| matches!(v.value, VariantFormat::Unit)) => {
            TypeBody::StringEnum {
                variants: variants.values().map(|v| EnumVariant { value: v.name.clone(), doc: None }).collect(),
            }
        }
        // serde-reflection reports an enum with data the way serde's default (external) tagging writes it.
        ContainerFormat::Enum(variants) => TypeBody::Alias {
            ty: Ty::Union {
                members: variants
                    .values()
                    .map(|v| match &v.value {
                        VariantFormat::Unit => Ty::Literal { value: v.name.clone() },
                        VariantFormat::NewType(inner) => Ty::Map {
                            key: Box::new(Ty::Literal { value: v.name.clone() }),
                            value: Box::new(ty_of(inner, module)),
                        },
                        _ => Ty::Unknown { reason: format!("variant `{}` of a foreign enum with tuple data", v.name) },
                    })
                    .collect(),
            },
        },
    }
}

impl<T: Serialize + DeserializeOwned + 'static> Shape for Foreign<T> {
    fn shape(reg: &mut Registry) -> Ty {
        let traced = traced::<T>();
        let module = reg.current_module().map(str::to_owned);
        for (name, container) in &traced.containers {
            let body = body_of(name, container, &traced.enums, module.as_deref());
            reg.named(name, None, std::any::type_name::<T>(), |_| Definition { doc: None, body });
        }
        ty_of(&traced.format, module.as_deref())
    }
}

impl<T: Serialize + DeserializeOwned + 'static> FromLuaArg for Foreign<T> {
    fn from_lua_arg(value: Value, lua: &Lua) -> Result<Self, String> {
        let shaped = crate::validate::shape_of::<Self>(Some("foreign"));
        crate::validate::check(&value, &shaped.0, &shaped.1, "")?;
        from_serde(value, lua).map(Foreign)
    }
}

impl<T: Serialize> IntoLuaRet for Foreign<T> {
    fn into_lua_ret(self, lua: &Lua, _: &'static str) -> mlua::Result<Value> {
        to_serde(&self.0, lua)
    }
}
