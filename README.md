# vsql-extension-template-rust

A [cargo-generate](https://github.com/cargo-generate/cargo-generate) template for building [VillageSQL](https://github.com/villagesql/vsql-rust-sdk) extensions in Rust.

## Prerequisites

- [Rust toolchain](https://rustup.rs) (stable)
- cargo-generate

```sh
cargo install cargo-generate
```

## Usage

```sh
cargo generate gh:villagesql/vsql-extension-template-rust --name vsql_my_extension
```

cargo-generate will prompt for:

| Prompt | Default | Description |
|--------|---------|-------------|
| Extension description | `A VillageSQL extension` | Populates `Cargo.toml` and `manifest.json` |
| Author name | — | Populates `manifest.json` |
| License | `GPL-2.0` | Populates `Cargo.toml` and `manifest.json` |
| SQL function name | — | The name of the initial SQL function (e.g. `rot13`, `my_func`) |

## What you get

```
vsql_my_extension/
├── .github/
│   └── workflows/
│       └── ci.yml          # fmt, clippy, audit, and cargo vsql test
├── Cargo.toml
├── manifest.json           # vsql extension manifest
├── src/
│   └── lib.rs              # starter STRING → STRING function
└── mysql-test/
    ├── suite.opt
    ├── t/
    │   └── vsql_my_extension.test
    └── r/
        └── vsql_my_extension.result
```

The generated `src/lib.rs` registers a working passthrough (identity) function under your chosen SQL function name. It compiles, passes `cargo vsql test`, and is ready to replace with your logic.

## Developing your extension

Install [cargo-vsql](https://github.com/villagesql/vsql-rust-sdk/tree/main/cargo-vsql):

```sh
cargo install cargo-vsql
```

Build and run tests against a local VillageSQL server:

```sh
export VillageSQL_BUILD_DIR=/path/to/villagesql/build
cargo vsql test
```

Record updated expected results after changing test queries:

```sh
cargo vsql test --record
```

See the [cargo-vsql README](https://github.com/villagesql/vsql-rust-sdk/tree/main/cargo-vsql) and the [villagesql SDK README](https://github.com/villagesql/vsql-rust-sdk/tree/main/villagesql) for the full API.

## CI

The generated workflow (`.github/workflows/ci.yml`) runs four jobs on every push and pull request:

| Job | What it does |
|-----|-------------|
| `fmt` | `cargo fmt --check` |
| `clippy` | `cargo clippy -- -D warnings` |
| `audit` | `cargo audit` via `rustsec/audit-check` |
| `test` | Builds, runs `cargo vsql test` against the VillageSQL dev server, and uploads the `.veb` artifact |

The `test` job uses [`villagesql/extension-actions/rust`](https://github.com/villagesql/extension-actions) and requires the `actions: read` permission to download the dev server artifact from `villagesql/villagesql-server`.

## Examples

The [vsql-rust-sdk examples](https://github.com/villagesql/vsql-rust-sdk/tree/main/examples) show what a complete extension looks like:

- [`vsql_rot13`](https://github.com/villagesql/vsql-rust-sdk/tree/main/examples/vsql_rot13) — minimal string function
- [`vsql_rational`](https://github.com/villagesql/vsql-rust-sdk/tree/main/examples/vsql_rational) — custom SQL type with arithmetic functions
