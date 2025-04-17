# Prisma Rust Schema

## Usage

1. Install the binary

```bash
cargo install prisma-rust-schema
```

2. Add the generator config to your `.prisma` file:

```prisma
generator prisma_rust_schema {
  provider = "prisma-rust-schema"
  output   = "./src/prisma"
}
```

3. Run the generator

```bash
npx prisma generate
```

4. Use the generated code in your Rust project:

```rust
use prisma::MyModel;
```

## Constraints

Currently, all enums and structs generated have `#[derive(Serialize, Deserialize, Debug)]` and `#[serde(rename_all = "camelCase")]` attributes. In future versions, this will be configurable.

## Development

```bash
npx prisma generate
```
