## DEEPWELL Integration Testing

Here, we use [Rust's integration testing](https://doc.rust-lang.org/rust-by-example/testing/integration_testing.html) to check the behavior of `deepwell` as a whole. This means that these tests lack access to its internal functions and **should call its JSONRPC API to affect change and verify outputs**. However, in rare cases where an assertion cannot be performed through other means, then use of the "service" structs for immutable operations is acceptable.

To give a simple example, we can test that basic page operations work through a test that calls the following:
1. `page_get` (assert page doesn't exist)
2. `page_create`
3. `page_get` (assert page does exist)
4. `page_edit`
5. `page_get` (assert page has new content)
6. `page_delete`
7. `page_get` (assert page doesn't exist)

### Quickstart Examples

If you're testing stateless methods (e.g. string translation, Caddyfile generation), then see `tests/locale.rs`.

If you're testing stateful methods (e.g. user creation, content filters), then see `tests/page.rs`.
