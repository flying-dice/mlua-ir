# mlua-ir

**Write a Lua API once, in plain Rust. Get the Luau and TypeScript types for free.**

mlua-ir reads an mlua API straight from its Rust signatures and turns it into one intermediate representation: a
JSON description of every module, function, table, userdata, error and stream a script can reach. Generators turn
that into `.d.luau` for luau-lsp and `.d.ts` for TypeScriptToLua. The declarations can't drift from the code,
because the code is the only place the API is written down.

## The problem

With plain mlua, an API is written three times: the Rust, the `.d.luau`, the `.d.ts`. The two declaration files are
kept up by hand, and they fall behind. Shapes end up described in doc comments, return types turn into `any`, and
errors become strings that scripts have to parse.

## The fix

```rust
#[mlua_ir::module("std:fs")]
pub mod fs {
    /// Why a file operation failed.
    #[derive(Debug, Serialize, thiserror::Error, LuaError)]
    #[serde(tag = "kind", rename_all = "snake_case")]
    pub enum FsError {
        #[error("no such file: {path}")]
        NotFound { path: String },
    }

    /// The text of a file.
    pub fn read(path: String) -> Result<String, FsError> { ... }
}
```

That is the whole module. Nothing is registered by hand, and no type is written twice. A script gets this:

```luau
read: (path: string) -> (string?, std_fs_FsError?)
```

The error kinds are a declared union, so `err.kind == "not_found"` narrows in both Luau and TypeScript.

## How it works

The macros only gather names, docs and signatures. Traits do the typing (`Shape`, `IntoReturn`, `FromLuaArg`), so
types resolve through generics and through other crates. bollard's Docker models come out as about 47 fully typed
records, with every enum variant, traced from their serde implementation.

Wrong input is refused at run time with the reason and its location:
`std:text.pad: argument #2 'width': expected a whole number from 0 to 4294967295, got -1`.

The one escape hatch, `Unknown<Reason>`, won't compile without saying why no shape exists.

## What it covers

Functions, globals and constants, typed tables, options tables with defaults, string and tagged unions, lists and
maps, multiple returns, `nil` for absence, returned and raised errors, userdata with fields, methods and
constructors, async functions, streams, callbacks, validated strings, other crates' types, and a diff between two
schemas that flags breaking changes.

## The proof

`bash tools/check.sh` runs these checks:

- luau-lsp checks the scripts in `lua/` and every doc example clean, and refuses every file in `lua/negative/`.
- tsc checks `ts/usage.ts`; every `@ts-expect-error` in `ts/negative.ts` must meet a real error.
- TypeScriptToLua compiles the TypeScript to Luau, and it runs in the same VM as the Luau scripts.
- `cargo test` runs every scenario, every doc example and 20 bad inputs that must be refused.

## Use it

```toml
[dependencies]
mlua-ir = { git = "https://github.com/JonathanTurnock/mlua-ir.git" }
```

Turn on `features = ["send"]` when your mlua has `send`. Luau only, for now.

```sh
cargo run -p mlua-ir-demo --bin schema   # write generated/: schema.json, types.d.luau, types.d.ts
```

## Layout

- `crates/mlua-ir`: the IR, the traits and the runtime
- `crates/mlua-ir-macros`: `#[module]`, `#[object]` and the derives
- `crates/mlua-ir-gen`: the Luau and TypeScript generators and the schema diff
- `demo/`: one working module per scenario, from a single function to a whole Docker client
