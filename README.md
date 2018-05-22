# Graphed

A proof-of-concept with GraphQL.

## Running

The `rust-toolchain` file sets the correct version of Rust (as of May 20,
2018). You can also run this command:

```bash
rustup default nightly-2018-05-16
```

Compile and run.

```bash
cargo run
```

[GraphiQL](https://github.com/graphql/graphiql) should be running on
<http://localhost:8000/>.

## TODOs

* [ ] authorization/authentication
* [ ] associate trees with the users who created them
* [ ] share trees
* [ ] edit nodes
* [ ] optimize queries
