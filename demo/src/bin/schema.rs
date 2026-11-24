//! `cargo run -p mlua-ir-demo --bin schema` writes everything that is read from the IR:
//!
//! - `generated/schema.json`: the IR itself
//! - `generated/types.d.luau`, `generated/types.d.ts`: the declarations, and one pair per module under `modules/`
//! - `generated/examples/*.luau`: the doc examples as scripts (run by the tests, checked by luau-lsp)
//! - `dist/mlua-types/`: the installable package of this runtime version
//!
//! `--check` writes nothing and fails when any of those is stale. `--diff <old schema.json>` prints what changed
//! since that schema and fails on a breaking change.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use mlua_ir::ir::Ir;
use mlua_ir_gen::{Level, diff, examples, luau, ts};

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).parent().expect("the demo crate is inside the workspace").to_path_buf()
}

fn file_stem(module: &str) -> String {
    module.replace([':', '.', '/'], "_")
}

/// Every output, as path → content.
fn outputs(ir: &Ir) -> BTreeMap<PathBuf, String> {
    let mut out = BTreeMap::new();
    let schema = serde_json::to_string_pretty(ir).expect("the IR serializes") + "\n";
    let luau_file = luau::file(ir);
    let ts_file = ts::file(ir);
    out.insert("generated/schema.json".into(), schema.clone());
    out.insert("generated/types.d.luau".into(), luau_file.clone());
    out.insert("generated/types.d.ts".into(), ts_file.clone());
    for m in &ir.modules {
        let stem = file_stem(&m.name);
        out.insert(format!("generated/modules/{stem}.d.luau").into(), luau::module_section(ir, m));
        out.insert(format!("generated/modules/{stem}.d.ts").into(), ts::module_section(ir, m));
    }
    for (i, example) in examples(ir).iter().enumerate() {
        out.insert(format!("generated/examples/{:02}_{}.luau", i + 1, file_stem(&example.source)).into(), example.code.clone());
    }
    let package = serde_json::json!({
        "name": "mlua-types",
        "version": ir.version,
        "description": "Luau and TypeScript declarations of the mlua-ir demo runtime, generated from its schema",
        "types": "index.d.ts",
        "files": ["index.d.ts", "types.d.luau", "schema.json"],
        "peerDependencies": { "@typescript-to-lua/language-extensions": "*" }
    });
    out.insert("dist/mlua-types/package.json".into(), serde_json::to_string_pretty(&package).expect("json") + "\n");
    out.insert("dist/mlua-types/index.d.ts".into(), ts_file);
    out.insert("dist/mlua-types/types.d.luau".into(), luau_file);
    out.insert("dist/mlua-types/schema.json".into(), schema);
    out
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ir = mlua_ir_demo::schema();
    if let Some(old) = args.iter().position(|a| a == "--diff").and_then(|i| args.get(i + 1)) {
        let old: Ir = serde_json::from_str(&std::fs::read_to_string(old).expect("the old schema reads")).expect("a schema");
        let changes = diff(&old, &ir);
        for change in &changes {
            println!("{change}");
        }
        let breaking = changes.iter().any(|c| c.level == Level::Breaking);
        println!("{} change(s) from {} to {}{}", changes.len(), old.version, ir.version, if breaking { ": BREAKING" } else { "" });
        return if breaking { ExitCode::FAILURE } else { ExitCode::SUCCESS };
    }
    let check = args.iter().any(|a| a == "--check");
    let root = root();
    let mut stale = Vec::new();
    let outputs = outputs(&ir);
    let count = outputs.len();
    for (path, text) in outputs {
        let full = root.join(&path);
        let current = std::fs::read_to_string(&full).ok().map(|t| t.replace("\r\n", "\n"));
        if current.as_deref() == Some(text.as_str()) {
            continue;
        }
        if check {
            stale.push(path);
        } else {
            std::fs::create_dir_all(full.parent().expect("a parent")).expect("the directory is made");
            std::fs::write(&full, text).expect("the file is written");
            println!("wrote {}", path.display());
        }
    }
    if stale.is_empty() {
        if check {
            println!("everything generated from the schema is up to date: {count} files");
            println!("{} modules, {} types, {} doc examples", ir.modules.len(), ir.types.len(), examples(&ir).len());
            for m in &ir.modules {
                println!("  {} ({} functions, {} constants, {} objects)", m.name, m.functions.len(), m.constants.len(), m.objects.len());
            }
        }
        ExitCode::SUCCESS
    } else {
        for path in stale {
            eprintln!("stale: {} (run `cargo run -p mlua-ir-demo --bin schema`)", path.display());
        }
        ExitCode::FAILURE
    }
}
