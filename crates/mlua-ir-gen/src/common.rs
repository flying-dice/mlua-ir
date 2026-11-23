//! What both backends share: which types a module owns, and the notes a generator adds to a doc.

use mlua_ir::ir::{Function, Ir, Module, ModuleKind, Ty, TypeDef};

/// The qualified name of a module's own type: `std:text` is `std_text`.
pub fn module_type(module: &Module) -> String {
    module.name.replace([':', '.', '-', '/'], "_")
}

/// The types a module owns, in name order.
pub fn owned_types<'a>(ir: &'a Ir, module: &Module) -> Vec<&'a TypeDef> {
    let owner = match module.kind {
        ModuleKind::Require => Some(module.name.as_str()),
        ModuleKind::Globals => None,
    };
    ir.types.iter().filter(|t| t.module.as_deref() == owner && (owner.is_some() || is_global_type(ir, t))).collect()
}

/// A type with no module belongs to the globals section when nothing else claims it.
fn is_global_type(_ir: &Ir, t: &TypeDef) -> bool {
    t.module.is_none()
}

/// The doc a declaration shows: the Rust doc, then what the signature says that a type cannot (yields, raises).
pub fn function_doc(f: &Function, raises: Option<String>) -> Option<String> {
    let mut lines: Vec<String> = f.doc.iter().map(ToString::to_string).collect();
    if f.is_async {
        lines.push("Yields the calling coroutine while it waits.".to_owned());
    }
    if let Some(raised) = raises {
        lines.push(raised);
    }
    let text = lines.join("\n\n");
    (!text.is_empty()).then_some(text)
}

/// Whether a type is already optional (so `?` is not written twice).
pub fn is_optional(ty: &Ty) -> bool {
    matches!(ty, Ty::Optional { .. } | Ty::Nil | Ty::Unknown { .. })
}

/// A JSON value as source text for a doc (`false`, `"x"`, `10`).
pub fn shown(value: &serde_json::Value) -> String {
    value.to_string()
}

pub fn is_identifier(name: &str) -> bool {
    let mut chars = name.chars();
    chars.next().is_some_and(|c| c.is_ascii_alphabetic() || c == '_') && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// A string literal, escaped the way both Luau and TypeScript read it.
pub fn quoted(text: &str) -> String {
    serde_json::Value::String(text.to_owned()).to_string()
}
