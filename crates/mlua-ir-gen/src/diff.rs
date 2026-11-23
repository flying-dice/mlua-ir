//! What changed between two published schemas, and whether it breaks a script written against the older one.
//!
//! Release CI diffs the new IR against the last published one: a breaking change without a major version bump
//! fails the release, and every addition gets its `@since` without anyone writing it.

use std::collections::BTreeMap;
use std::fmt;

use mlua_ir::ir::{Function, Ir, TypeBody, TypeDef};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    /// Something new: a script written against the older schema still checks.
    Addition,
    /// A script written against the older schema may no longer check or run.
    Breaking,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Change {
    pub level: Level,
    pub what: String,
}

impl fmt::Display for Change {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let level = match self.level {
            Level::Addition => "addition",
            Level::Breaking => "BREAKING",
        };
        write!(f, "{level}: {}", self.what)
    }
}

fn functions(ir: &Ir) -> BTreeMap<String, &Function> {
    let mut out = BTreeMap::new();
    for m in &ir.modules {
        for f in &m.functions {
            out.insert(format!("{}.{}", m.name, f.name), f);
        }
    }
    for t in &ir.types {
        if let TypeBody::Object { methods, statics, .. } = &t.body {
            for f in methods {
                out.insert(format!("{}:{}", t.name, f.name), f);
            }
            for f in statics {
                out.insert(format!("{}.{}", t.name, f.name), f);
            }
        }
    }
    out
}

fn types(ir: &Ir) -> BTreeMap<&str, &TypeDef> {
    ir.types.iter().map(|t| (t.name.as_str(), t)).collect()
}

pub fn diff(old: &Ir, new: &Ir) -> Vec<Change> {
    let mut changes = Vec::new();
    let (old_fns, new_fns) = (functions(old), functions(new));
    for (name, f) in &old_fns {
        match new_fns.get(name) {
            None => changes.push(Change { level: Level::Breaking, what: format!("`{name}` was removed") }),
            Some(g) if f.params != g.params || f.returns != g.returns || f.raises != g.raises => {
                let level = if added_optional_params(f, g) { Level::Addition } else { Level::Breaking };
                changes.push(Change { level, what: format!("`{name}` changed its signature") });
            }
            Some(_) => {}
        }
    }
    for name in new_fns.keys().filter(|n| !old_fns.contains_key(*n)) {
        changes.push(Change { level: Level::Addition, what: format!("`{name}` was added") });
    }
    let (old_types, new_types) = (types(old), types(new));
    for (name, t) in &old_types {
        match new_types.get(name) {
            None => changes.push(Change { level: Level::Breaking, what: format!("type `{name}` was removed") }),
            Some(u) if t.body != u.body => {
                changes.push(Change { level: type_change_level(&t.body, &u.body), what: format!("type `{name}` changed") });
            }
            Some(_) => {}
        }
    }
    for name in new_types.keys().filter(|n| !old_types.contains_key(*n)) {
        changes.push(Change { level: Level::Addition, what: format!("type `{name}` was added") });
    }
    changes.sort_by(|a, b| b.level.cmp(&a.level).then(a.what.cmp(&b.what)));
    changes
}

/// The new signature only appends optional parameters to the old one.
fn added_optional_params(old: &Function, new: &Function) -> bool {
    old.returns == new.returns
        && old.raises == new.raises
        && new.params.len() > old.params.len()
        && new.params[..old.params.len()] == old.params[..]
        && new.params[old.params.len()..].iter().all(|p| p.optional)
}

/// Adding an optional field to an options table, or a member to a string enum a script only reads, is an
/// addition; anything else is treated as breaking.
fn type_change_level(old: &TypeBody, new: &TypeBody) -> Level {
    match (old, new) {
        (TypeBody::Options { fields: a }, TypeBody::Options { fields: b })
            if b.len() > a.len() && b[..a.len()] == a[..] =>
        {
            Level::Addition
        }
        (TypeBody::Object { fields: fa, methods: ma, statics: sa }, TypeBody::Object { fields: fb, methods: mb, statics: sb })
            if fa.iter().all(|f| fb.contains(f)) && ma.iter().all(|m| mb.contains(m)) && sa.iter().all(|s| sb.contains(s)) =>
        {
            Level::Addition
        }
        _ => Level::Breaking,
    }
}
