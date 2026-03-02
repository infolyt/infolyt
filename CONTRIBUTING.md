# Contributing to Infolyt

## Before your first PR
Please sign our CLA at https://cla-assistant.io/infolyt/infolyt
(takes 30 seconds, required once only)

## Development setup
\`\`\`bash
git clone https://github.com/infolyt/infolyt
cd infolyt
cargo build          # verify everything compiles
cargo test           # run test suite
just check           # run all CI checks locally
\`\`\`

## Finding issues to work on
- [`good first issue`](../../labels/good%20first%20issue) — great starting points
- [`help wanted`](../../labels/help%20wanted) — we'd love your help
- [`bug`](../../labels/bug) — confirmed bugs

## Pull request checklist
- [ ] CLA signed
- [ ] `cargo fmt` applied
- [ ] `cargo clippy` passes with no warnings
- [ ] Tests added or updated
- [ ] CHANGELOG.md updated under `[Unreleased]`

## Code style
- Run `just fmt` before committing
- All public APIs must have doc comments
- No `unwrap()` in library code — use `?` and `anyhow`