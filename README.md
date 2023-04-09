dev

```sh
#1. push or pull prisma/schema.prisma
cargo run -p prisma-cli -- db push
#cargo run -p prisma-cli -- db pull

#2. generate src/db.rs (from prisma/schema.prisma)
cargo run -p prisma-cli -- generate

#3 run api (debug mode)
cargo run

```

prod

```rs
cargo build --release
cargo run --release
```
