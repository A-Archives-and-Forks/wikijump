## DEEPWELL Integration Testing

Here, we use [Rust's integration testing](https://doc.rust-lang.org/rust-by-example/testing/integration_testing.html) to check the behavior of `deepwell` as a whole. This means that these tests lack access to its internal functions and **should call its JSONRPC API to affect change and verify outputs**. However, using "service" structs for immutable operations as part of test state assertions is acceptable.

To give a simple example, we can test that basic page operations work through a test that calls the following:
1. `page_get` (assert page doesn't exist)
2. `page_create`
3. `page_get` (assert page does exist)
4. `page_edit`
5. `page_get` (assert page has new content)
6. `page_delete`
7. `page_get` (assert page doesn't exist)

### Requirements

Integration testing runs real deepwell processing code when it receives requests, and so needs a local database, Redis/Valkey instance, and S3-compatible object store. All changes to the database are reverted, but reverting changes to the remaining two are not yet implemented. The code assumes that database migrations and seeding have already been run.

If you are running Wikijump locally, then all three are provided for you, and you should be able to run `cargo test` in the crate root with no other modifications. (Or more specifically, it only needs `wikijump-database-1`, `wikijump-cache-1`, `wikijump-files-1`)

These tests are also configured to run in a [GitHub Workflow](../../.github/workflows/deepwell.yaml), and will be kicked off when you make a PR that changes something in deepwell.

### Quickstart Examples

If you're testing stateless methods (e.g. string translation, Caddyfile generation), then see [`tests/locale.rs`](./locale.rs).

If you're testing stateful methods (e.g. user creation, content filters), then see [`tests/page.rs`](./page.rs).

### Internals

Integration tests work by constructing a local `TestRunner` instance, which contains the `ServerState`, and starts a database transaction to contain all of the test's database changes. This instance can expose its `ServerContext` via the `.context()` method. This corresponds with the wrapping each JSONRPC method receives.

This database transaction is automatically rolled back when the test ends, regardless of whether or not the test itself passed. It uses the built-in configuration specified by `Config::integration_testing()`.

You can initialize this environment by running `TestRunner::setup().await` at the start of your integration test. This is found in the `common` module, which also has a series of useful helper functions and macros. It also contains `common::IP_ADDRESS` as a dummy value to pass in for JSONRPC requests which require the caller's IP address.
