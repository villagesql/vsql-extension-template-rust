# Contributing

Thanks for your interest in improving this template. This document covers contributions to the **template itself** — not extensions generated from it.

## Reporting issues

Open an issue at https://github.com/villagesql/vsql-extension-template-rust/issues with:

- What you tried (`cargo generate ...` invocation and prompt answers)
- What happened vs. what you expected
- Output of `cargo --version` and `rustc --version`

## Submitting changes

1. Fork and create a feature branch.
2. Make your change. If it affects the generated project, regenerate locally and confirm `cargo fmt --check`, `cargo clippy -- -D warnings`, and `cargo vsql test` still pass.
3. Open a pull request describing the motivation and any user-visible impact. Pull requests must include signing the VillageSQL Contributor License Agreement (CLA) before they can be merged.

## Testing the template locally

Generate a throwaway extension from your working copy and run its checks:

```sh
cargo generate --path . --name vsql_smoketest
cd vsql_smoketest
cargo fmt --check
cargo clippy -- -D warnings
cargo vsql test
```

## Scope

Keep the template minimal. It is a starting point, not a kitchen sink — features that belong in user extensions or in the [vsql-rust-sdk](https://github.com/villagesql/vsql-rust-sdk) should live there instead.
