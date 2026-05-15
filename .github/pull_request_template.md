## What does this PR do?

<!-- A clear, concise description of the change -->

## Type of change

- [ ] Bug fix
- [ ] New contract function
- [ ] Test coverage
- [ ] Documentation update
- [ ] Refactor / chore

## Related issue

Closes #<!-- issue number -->

## Checklist

- [ ] `cargo build --target wasm32-unknown-unknown --release` passes
- [ ] `cargo test --features testutils` passes
- [ ] `cargo clippy -- -D warnings` passes
- [ ] `cargo fmt` applied
- [ ] New functions have `///` doc comments
- [ ] New write functions call `require_auth()` where appropriate
- [ ] Events are emitted for state changes
- [ ] Security implications considered and noted below

## Security notes

<!-- Describe any auth, state mutation, or trust boundary considerations -->
