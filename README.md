# bun vs axum sqlite test

Run axum sqlite test:

```sh
cargo build --release
./target/release/axum-test # listening on port 9001
```

Run bun test

```sh
cd bun-test
bun run src/index.ts # listening on port 9001
```