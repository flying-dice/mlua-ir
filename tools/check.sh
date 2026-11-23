#!/usr/bin/env bash
# Everything that proves the DX: the Rust side, the generated files, Luau's checker and TypeScript's, and the
# TypeScript backend compiled to Luau and run.
set -euo pipefail
cd "$(dirname "$0")/.."

LUAU_LSP="${LUAU_LSP:-luau-lsp}"
DEFS="--definitions:@mlua=generated/types.d.luau"
analyze() { "$LUAU_LSP" analyze --formatter=plain --platform=standard "$DEFS" "$@" 2>&1 | grep -v '^\[INFO\]' || true; }

echo "== the generated files are the schema's"
cargo run -q -p mlua-ir-demo --bin schema -- --check

echo "== luau-lsp: every scenario script and doc example checks clean"
found="$(analyze lua/*.luau generated/examples/*.luau)"
if [ -n "$found" ]; then echo "$found"; exit 1; fi

echo "== luau-lsp: every negative fixture is refused"
for f in lua/negative/*.luau; do
  if ! analyze "$f" | grep -q 'TypeError'; then echo "expected a type error in $f"; exit 1; fi
done

echo "== tsc: usage checks, every @ts-expect-error meets an error"
./node_modules/.bin/tsc -p ts

echo "== tstl: the TypeScript usage compiles to Luau"
./node_modules/.bin/tstl -p ts/tsconfig.tstl.json

echo "== cargo test: scripts, doc examples, refusals, the compiled TypeScript, the release diff"
MLUA_IR_REQUIRE_TS=1 cargo test -q --workspace

echo "== cargo test: the macros under mlua's \`send\` (as neoc uses it)"
cargo test -q -p mlua-ir --features send

echo "== clippy"
cargo clippy -q --workspace --all-targets -- -D warnings
cargo clippy -q -p mlua-ir --all-targets --features send -- -D warnings

echo "all checks passed"
