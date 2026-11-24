//! The intermediate representation: everything a script can reach, as data.
//!
//! This is the contract between the Rust side and every generator. A generator (`mlua-ir-gen`'s Luau and
//! TypeScript backends, or anything else: JSON Schema, MCP tool schemas, a docs site) reads an [`Ir`] and nothing
//! else. It serializes to JSON (`schema.json`), so a generator does not have to be written in Rust.

use serde::{Deserialize, Serialize};

/// A whole runtime's surface.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ir {
    /// The runtime version the surface belongs to: the version installable types are published under.
    pub version: String,
    /// The modules, sorted by name. Globals are a module of kind [`ModuleKind::Globals`].
    pub modules: Vec<Module>,
    /// Every named type any module reaches, sorted by name.
    pub types: Vec<TypeDef>,
}

impl Ir {
    pub fn type_def(&self, name: &str) -> Option<&TypeDef> {
        self.types.iter().find(|t| t.name == name)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModuleKind {
    /// Reached with `require("<name>")`.
    Require,
    /// Installed as globals.
    Globals,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Module {
    /// `std:text`; for globals, the Rust module's name.
    pub name: String,
    pub kind: ModuleKind,
    pub doc: Option<String>,
    pub functions: Vec<Function>,
    pub constants: Vec<Constant>,
    /// Userdata types whose statics (constructors) the module exports under their own name (`collections.Counter`).
    pub objects: Vec<ObjectExport>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjectExport {
    /// The name in the module (`Counter`).
    pub name: String,
    /// The qualified type (`std_collections_Counter`); its statics are `<type>Statics`.
    pub type_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Constant {
    pub name: String,
    pub doc: Option<String>,
    pub ty: Ty,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Function {
    pub name: String,
    pub doc: Option<String>,
    pub params: Vec<Param>,
    pub returns: Returns,
    /// What the call raises (rather than returns) when it fails expectedly; `None` when it raises only for bugs.
    pub raises: Option<Ty>,
    /// The call yields the script's coroutine while it waits.
    pub is_async: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Param {
    pub name: String,
    pub ty: Ty,
    /// The argument may be left out (`Option<T>`, an options table).
    pub optional: bool,
    /// The last parameter takes every remaining argument, `nil`s included (`mlua_ir::Variadic<T>`).
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub variadic: bool,
}

/// What a call gives back.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Returns {
    /// Nothing.
    None,
    /// One value.
    One { ty: Ty },
    /// Several values (a Rust tuple).
    Many { tys: Vec<Ty> },
    /// `(value, nil)` or `(nil, error)`: an expected failure is a returned value.
    Fallible { ok: Ty, err: Ty },
}

/// A type in a position: a parameter, a field, a return.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Ty {
    Nil,
    Boolean,
    /// A whole number in `min..=max` (Lua has one number type; the range is enforced on the way in).
    Integer { min: i64, max: u64 },
    Number,
    String,
    /// The one allowed "anything": the shape is decided at run time by data no one controls. Always says why.
    Unknown { reason: String },
    Optional { inner: Box<Ty> },
    List { item: Box<Ty> },
    Map { key: Box<Ty>, value: Box<Ty> },
    /// A string literal (a member of a union).
    Literal { value: String },
    /// One of several types.
    Union { members: Vec<Ty> },
    /// A named type: see [`Ir::types`].
    Named { name: String },
    /// A Lua function (a callback).
    Function { params: Vec<Param>, returns: Box<Returns> },
    /// A stream of `item`s that may end in `error`: the shared `Stream<T, E>` type.
    Stream { item: Box<Ty>, error: Box<Ty> },
}

impl Ty {
    pub fn optional(inner: Ty) -> Ty {
        match inner {
            already @ Ty::Optional { .. } => already,
            inner => Ty::Optional { inner: Box::new(inner) },
        }
    }

    pub fn named(name: impl Into<String>) -> Ty {
        Ty::Named { name: name.into() }
    }

    /// The name of a named type, if this is one.
    pub fn name(&self) -> Option<&str> {
        match self {
            Ty::Named { name } => Some(name),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypeDef {
    /// Qualified: `std_geo_Point`.
    pub name: String,
    /// The name the module exports it under: `Point`.
    pub short_name: String,
    /// The module that owns it, if any.
    pub module: Option<String>,
    /// The Rust path it was read from, for diagnostics.
    pub rust: String,
    pub doc: Option<String>,
    pub body: TypeBody,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum TypeBody {
    /// A table with known fields.
    Record { fields: Vec<Field> },
    /// A trailing options table: every field may be left out, and takes its default.
    Options { fields: Vec<Field> },
    /// One of a set of strings.
    StringEnum { variants: Vec<EnumVariant> },
    /// Tables told apart by the string in `tag`.
    TaggedUnion { tag: String, variants: Vec<Variant> },
    /// An error: a tagged union on `kind`, every variant also carrying `common` (`message`, `op`).
    Error { tag: String, variants: Vec<Variant>, common: Vec<Field> },
    /// A userdata: fields read through getters, `:` methods, and `.` statics (constructors).
    Object { fields: Vec<Field>, methods: Vec<Function>, statics: Vec<Function> },
    /// Another name for a type (a validated string, a union of accepted types).
    Alias { ty: Ty },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub doc: Option<String>,
    pub ty: Ty,
    /// The value a missing field takes (options tables).
    pub default: Option<serde_json::Value>,
    /// Scripts can read it but not write it (userdata getters).
    pub readonly: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnumVariant {
    pub value: String,
    pub doc: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Variant {
    /// The value of the tag field for this variant.
    pub tag_value: String,
    pub doc: Option<String>,
    pub fields: Vec<Field>,
}
