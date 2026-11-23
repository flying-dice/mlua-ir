//! The ```` ```luau ```` examples in the docs, as scripts: run by the tests and checked by luau-lsp, so a doc
//! example that stops working fails the build.

use mlua_ir::ir::{Ir, ModuleKind, TypeBody};

pub struct Example {
    /// Where it was written: `std:text.pad`.
    pub source: String,
    /// A `--!strict` script that requires the module under its short name, then the example.
    pub code: String,
}

fn blocks(doc: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut current: Option<Vec<&str>> = None;
    for line in doc.lines() {
        let trimmed = line.trim();
        match &mut current {
            None if trimmed == "```luau" => current = Some(Vec::new()),
            Some(lines) if trimmed == "```" => {
                out.push(lines.join("\n"));
                current = None;
            }
            Some(lines) => lines.push(line),
            None => {}
        }
    }
    out
}

pub fn examples(ir: &Ir) -> Vec<Example> {
    let mut out = Vec::new();
    for m in &ir.modules {
        let prelude = match m.kind {
            ModuleKind::Require => {
                let short = m.name.rsplit(':').next().unwrap_or(&m.name);
                format!("--!strict\nlocal {short} = require(\"{}\")\n", m.name)
            }
            ModuleKind::Globals => "--!strict\n".to_owned(),
        };
        let mut docs: Vec<(String, &str)> = Vec::new();
        if let Some(d) = &m.doc {
            docs.push((m.name.clone(), d));
        }
        for f in &m.functions {
            if let Some(d) = &f.doc {
                docs.push((format!("{}.{}", m.name, f.name), d));
            }
        }
        for t in ir.types.iter().filter(|t| t.module.as_deref() == Some(m.name.as_str())) {
            if let Some(d) = &t.doc {
                docs.push((t.name.clone(), d));
            }
            if let TypeBody::Object { methods, statics, .. } = &t.body {
                for f in methods.iter().chain(statics) {
                    if let Some(d) = &f.doc {
                        docs.push((format!("{}.{}", t.name, f.name), d));
                    }
                }
            }
        }
        for (source, doc) in docs {
            for block in blocks(doc) {
                out.push(Example { source: source.clone(), code: format!("{prelude}{block}\n") });
            }
        }
    }
    out
}
