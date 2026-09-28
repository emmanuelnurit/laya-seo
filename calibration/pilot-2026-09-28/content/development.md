# Development

## Development

```bash
git clone https://github.com/AkashPriyadarshii/jev-seo.git
cd jev-seo
cargo test
cargo clippy --all-targets -- -D warnings
cargo run -- audit docs/ --min-pass 40
```

CI runs `cargo check --all-targets`, `cargo test --all`, and the SEO gate on every push to `master`.

---