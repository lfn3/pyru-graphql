### Pyru-graphql

Quick demo of getting python and rust to communicate with graphql.

The main point of this is to show how we can validate a graphql query
written or executed from python against a server written in rust.

This is intended for use in the context of a monorepo, where both the
server and a client source will be updated at the same time if one needs 
to change. Ideally, we'd like to catch any breaking changes at test time,
and preferably without requiring users to run a build step on their machine.

This is done by writing an `example` in rust that prints the 
schema from the server to stdout. We can use this from python by using
`subprocess.run` to call `cargo run --example emit_schema.rs`.
We do this in `tests/conftest`, and then stuff the resulting schema
into a client that can be used to validate queries.
