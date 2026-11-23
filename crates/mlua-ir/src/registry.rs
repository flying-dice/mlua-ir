//! Collects named types while a module's shape is read.

use std::collections::HashMap;

use crate::ir::{Ty, TypeBody, TypeDef};

/// Where a named type's definition is gathered. A type registers itself the first time it is reached, so a
/// module's types are exactly the ones its functions can reach.
#[derive(Default)]
pub struct Registry {
    types: Vec<TypeDef>,
    index: HashMap<String, usize>,
    module: Option<String>,
}

/// What a named type says of itself.
pub struct Definition {
    pub doc: Option<String>,
    pub body: TypeBody,
}

impl Registry {
    pub fn new() -> Self {
        Self::default()
    }

    /// The module whose shape is being read. Types that say no module of their own (another crate's) join it.
    pub fn current_module(&self) -> Option<&str> {
        self.module.as_deref()
    }

    pub fn enter_module(&mut self, name: &str) {
        self.module = Some(name.to_owned());
    }

    pub fn leave_module(&mut self) {
        self.module = None;
    }

    /// The qualified name of `short` in `module`: `std:geo` + `Point` is `std_geo_Point`.
    pub fn qualify(module: Option<&str>, short: &str) -> String {
        match module {
            Some(module) => format!("{}_{short}", module.replace([':', '.', '-', '/'], "_")),
            None => short.to_owned(),
        }
    }

    /// Registers a named type (once) and returns a reference to it. `define` runs only the first time; a
    /// placeholder is in place while it runs, so a type that reaches itself terminates.
    pub fn named(
        &mut self,
        short: &str,
        module: Option<&str>,
        rust: &str,
        define: impl FnOnce(&mut Registry) -> Definition,
    ) -> Ty {
        let module = module.map(str::to_owned).or_else(|| self.module.clone());
        let name = Self::qualify(module.as_deref(), short);
        if self.index.contains_key(&name) {
            return Ty::named(name);
        }
        let slot = self.types.len();
        self.index.insert(name.clone(), slot);
        self.types.push(TypeDef {
            name: name.clone(),
            short_name: short.to_owned(),
            module: module.clone(),
            rust: rust.to_owned(),
            doc: None,
            body: TypeBody::Alias { ty: Ty::Nil },
        });
        let definition = define(self);
        let def = &mut self.types[slot];
        def.doc = definition.doc;
        def.body = definition.body;
        Ty::named(name)
    }

    pub fn into_types(self) -> Vec<TypeDef> {
        self.types
    }
}
