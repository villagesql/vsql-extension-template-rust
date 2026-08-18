# {{project-name}}

{{description}}

**Docs:** [VillageSQL documentation](https://villagesql.com/docs) ·
[Writing extensions in Rust](https://villagesql.com/docs/guides/rust-extensions) ·
[Install VillageSQL Server](https://villagesql.com/install)

## Usage

```sql
INSTALL EXTENSION {{crate_name}};
SELECT {{sql_func_name}}('hello');
```

## Building

You need a VillageSQL build directory and
[cargo-vsql](https://github.com/villagesql/vsql-rust-sdk/tree/main/cargo-vsql):

```sh
cargo install cargo-vsql
export VillageSQL_BUILD_DIR=/path/to/villagesql/build
cargo vsql package
```

That writes the `.veb` to `dist/`. To load it into a local server:

```sh
cargo vsql install
```

## Testing

```sh
cargo vsql test
```

Re-record the expected results after you change a test query:

```sh
cargo vsql test --record
```

## License

{{license}}
