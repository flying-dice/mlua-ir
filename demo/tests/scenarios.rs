//! Every scenario runs for real: the scripts in `lua/`, the doc examples, the values the declarations would refuse,
//! and the generated files against the schema they come from.

use std::path::{Path, PathBuf};

use mlua_ir::ir::{Ir, Param, Ty};
use mlua_ir_gen::{Level, diff, examples};

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).parent().expect("the workspace").to_path_buf()
}

fn block_on<F: std::future::Future>(f: F) -> F::Output {
    tokio::runtime::Builder::new_current_thread().enable_time().build().expect("a runtime").block_on(f)
}

/// Runs a script in a fresh state with every module installed.
fn run(name: &str, code: &str) -> mlua::Result<()> {
    block_on(async {
        let lua = mlua_ir_demo::lua()?;
        mlua_ir_demo::run(&lua, name, code).await
    })
}

fn luau_files(dir: &Path) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = std::fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("{}: {e}", dir.display()))
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|e| e == "luau"))
        .collect();
    files.sort();
    files
}

#[test]
fn every_scenario_script_runs() {
    let files = luau_files(&root().join("lua"));
    assert_eq!(files.len(), 19, "one script per scenario 01..19");
    for file in files {
        let code = std::fs::read_to_string(&file).expect("the script reads");
        let name = file.file_name().expect("a name").to_string_lossy().into_owned();
        if let Err(e) = run(&name, &code) {
            panic!("{name} failed:\n{e}");
        }
    }
}

#[test]
fn every_doc_example_runs() {
    let ir = mlua_ir_demo::schema();
    let examples = examples(&ir);
    assert!(examples.len() >= 20, "the docs carry their examples");
    for example in examples {
        if let Err(e) = run(&example.source, &example.code) {
            panic!("the example of {} failed:\n{}\n{e}", example.source, example.code);
        }
    }
}

/// What a script that is not type-checked (or that lies to the checker) is told at run time.
#[test]
fn what_the_declarations_refuse_is_refused_at_run_time() {
    let cases: &[(&str, &str)] = &[
        (r#"require("std:text").pad("x", -1)"#, "std:text.pad: argument #2 'width': expected a whole number from 0 to 4294967295, got -1"),
        (r#"require("std:text").pad("x", 1.5)"#, "argument #2 'width': expected a whole number"),
        (r#"require("std:text").pad("x", "3")"#, "argument #2 'width': expected a whole number from 0 to 4294967295, got \"3\""),
        (r#"require("std:text").pad("x", 3, "extra")"#, "std:text.pad: takes 2 arguments, got more"),
        (r#"require("std:geo").distance({ x = 0, y = 0, lable = "a" }, { x = 0, y = 0 })"#, "unknown field `lable`"),
        (r#"require("std:geo").distance({ x = 0 }, { x = 0, y = 0 })"#, "missing field `y`"),
        (r#"require("std:dir").list("src", { recursve = true })"#, "unknown field `recursve`"),
        (r#"require("std:dir").list("src", { max_depth = "5" })"#, "max_depth: expected a whole number from 0 to 4294967295, got \"5\""),
        (r#"require("std:geo").distance({ x = 0, y = "1" }, { x = 0, y = 0 })"#, "y: expected a number, got \"1\""),
        (r#"require("std:log").write("loud", "x")"#, "expected one of \"debug\", \"info\", \"warn\", \"error\", got \"loud\""),
        (r#"require("std:draw").area({ kind = "triangle" })"#, "`kind` must be one of \"circle\", \"rect\""),
        (r#"require("std:draw").area({ kind = "circle", r = 1, w = 2 })"#, "unknown field `w`, expected one of r"),
        (r#"require("vnd:docker_models").line({ Names = { first = "/x" } })"#, "Names: a list has only the keys 1, 2, 3, ...; found the key `first`"),
        (r#"require("std:labels").keys({ 1, 2 })"#, "argument #1 'labels'"),
        (r#"require("std:labels").parse({ [1] = "a=b", [3] = "c=d" })"#, "a list has no holes; item 2 is missing"),
        (r#"require("std:collections").Counter.new():add("x")"#, "std:collections.Counter:add: argument #1 'n'"),
        (r#"require("std:time").Duration.seconds(1):plus({})"#, "expected a Duration"),
        (r#"require("std:time").describe("soon")"#, "expected a Duration, got \"soon\"; or expected a number"),
        (r#"require("std:names").describe("-bad")"#, "`-bad` is not a container name"),
        (r#"require("vnd:docker_models").line({ Nmaes = { "/x" } })"#, "unknown field `Nmaes`, expected one of Id, Names"),
        (r#"require("vnd:docker_models").line({ State = "sleeping" })"#, "State: expected one of \"\", \"created\", \"running\""),
        (r#"require("std:math2").TAU = 3"#, "readonly"),
        (r#"require("std:collections").Counter.new().increment()"#, "bad argument `self` to `Counter.increment`"),
        (r#"require("std:blob").to_hex(1)"#, "std:blob.to_hex: argument #1 'data': expected a string, got 1"),
        (r#"require("nope:module")"#, "no module named \"nope:module\""),
    ];
    for (code, expected) in cases {
        match run("refusal", code) {
            Ok(()) => panic!("`{code}` was not refused"),
            Err(e) => {
                let text = e.to_string();
                assert!(text.contains(expected), "`{code}`\n  expected: {expected}\n  got: {text}");
            }
        }
    }
}

#[test]
fn a_foreign_body_is_checked_before_it_is_read() {
    let code = r#"
        local bollard = require("vnd:bollard")
        local docker = assert(bollard.connect())
        docker:create_container("x", { Image = "nginx", HostConfig = { RestartPolicy = { Name = "sometimes" } } })
    "#;
    let err = run("body", code).expect_err("a restart policy Docker does not have");
    assert!(err.to_string().contains("HostConfig.RestartPolicy.Name: expected one of"), "{err}");
}

#[test]
fn the_generated_files_are_the_schema_s() {
    let ir = mlua_ir_demo::schema();
    let read = |p: &str| std::fs::read_to_string(root().join(p)).expect("run `cargo run -p mlua-ir-demo --bin schema`").replace("\r\n", "\n");
    let on_disk: Ir = serde_json::from_str(&read("generated/schema.json")).expect("a schema");
    assert_eq!(on_disk, ir, "generated/schema.json is stale: run `cargo run -p mlua-ir-demo --bin schema`");
    assert_eq!(read("generated/types.d.luau"), mlua_ir_gen::luau::file(&ir));
    assert_eq!(read("generated/types.d.ts"), mlua_ir_gen::ts::file(&ir));
}

#[test]
fn the_schema_reads_every_shape() {
    let ir = mlua_ir_demo::schema();
    let names: Vec<&str> = ir.modules.iter().map(|m| m.name.as_str()).collect();
    for expected in ["std:text", "globals", "std:geo", "vnd:bollard", "lib:json", "vnd:docker_models"] {
        assert!(names.contains(&expected), "{expected} is in {names:?}");
    }
    let unknowns: Vec<String> = all_unknown_reasons(&ir);
    assert_eq!(
        unknowns,
        vec![
            "any plain data a script built: tables, strings, numbers, booleans".to_owned(),
            "whatever JSON text was parsed: its shape is the text's, not this API's".to_owned(),
            "whatever `pcall` caught: any value can be raised".to_owned(),
            "whatever the script passed: it comes back as it is, the same table or function".to_owned(),
        ],
        "every unknown says why, and there are no others"
    );
}

fn all_unknown_reasons(ir: &Ir) -> Vec<String> {
    let json = serde_json::to_value(ir).expect("json");
    let mut found = Vec::new();
    walk(&json, &mut found);
    found.sort();
    found.dedup();
    found
}

fn walk(value: &serde_json::Value, found: &mut Vec<String>) {
    match value {
        serde_json::Value::Object(map) => {
            if map.get("kind").and_then(|k| k.as_str()) == Some("unknown")
                && let Some(reason) = map.get("reason").and_then(|r| r.as_str())
            {
                found.push(reason.to_owned());
            }
            map.values().for_each(|v| walk(v, found));
        }
        serde_json::Value::Array(items) => items.iter().for_each(|v| walk(v, found)),
        _ => {}
    }
}

#[test]
fn a_release_diff_tells_additions_from_breaks() {
    let new = mlua_ir_demo::schema();
    let mut old = new.clone();
    old.version = "0.0.9".into();
    let text = old.modules.iter_mut().find(|m| m.name == "std:text").expect("std:text");
    // the older release had a `pad` without `width` and a `shout` that is gone now
    let pad = text.functions.iter_mut().find(|f| f.name == "pad").expect("pad");
    pad.params.truncate(1);
    let mut shout = pad.clone();
    shout.name = "shout".into();
    text.functions.push(shout);
    // and `std:dir.list` took no options yet
    let dir = old.modules.iter_mut().find(|m| m.name == "std:dir").expect("std:dir");
    dir.functions[0].params.truncate(1);

    let changes = diff(&old, &new);
    let shown: Vec<String> = changes.iter().map(ToString::to_string).collect();
    assert!(shown.contains(&"BREAKING: `std:text.shout` was removed".to_owned()), "{shown:#?}");
    assert!(shown.contains(&"BREAKING: `std:text.pad` changed its signature".to_owned()), "{shown:#?}");
    assert!(shown.contains(&"addition: `std:dir.list` changed its signature".to_owned()), "{shown:#?}");
    assert!(changes.iter().any(|c| c.level == Level::Breaking));
    let _ = Param { name: String::new(), ty: Ty::Nil, optional: false };
}

/// `ts/usage.ts`, compiled to Luau by TypeScriptToLua (`bun run build:ts`), runs in the same VM. `tools/check.sh`
/// builds it first and sets `MLUA_IR_REQUIRE_TS`, so there it cannot be skipped.
#[test]
fn the_typescript_backend_compiled_by_tstl_runs() {
    let path = root().join("ts/out/usage.lua");
    let Ok(code) = std::fs::read_to_string(&path) else {
        assert!(std::env::var_os("MLUA_IR_REQUIRE_TS").is_none(), "{} is missing: run `bun run build:ts`", path.display());
        eprintln!("skipped: {} is not built (`bun run build:ts`)", path.display());
        return;
    };
    if let Err(e) = run("usage.lua", &code) {
        panic!("ts/out/usage.lua failed:\n{e}");
    }
}
