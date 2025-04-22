# Prisma Rust Schema

## Usage

### Binary

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

### Library

This crate exports the internal TypeScript types from [`prisma/prisma`](https://github.com/prisma/prisma/).

## Options

| Option                           | Example                                        | Description                                                                                     |
| -------------------------------- | ---------------------------------------------- | ----------------------------------------------------------------------------------------------- |
| `@prs.rename = <new_name>`       | `@prs.rename = username`                       | Rename the field in the generated Rust struct.                                                  |
| `@prs.skip`                      | `@prs.skip`                                    | Skip the field in the generated Rust struct.                                                    |
| `@prs.type = <type_override>`    | `@prs.type = usize`                            | Override the type of the field in the generated Rust struct.                                    |
| `@prs.visibility = <visibility>` | `@prs.visibility = public`                     | Override the visibility (public, private, protected) of the field in the generated Rust struct. |
| `@prs.derive = <trait>`          | `@prs.derive = Debug,Clone,serde::Deserialize` | Fully-qualified, comma-separated derive attributes for the generated Rust struct.               |

### Example

```prisma
/// User model documentation
/// @prs.visibility = protected
/// @prs.derive = Debug,Clone,serde::Deserialize,serde::Serialize
model User {
  /// User ID
  /// @prs.rename = `user_id`
  /// @prs.type = `usize`
  /// @prs.derive = `Debug,Clone,serde::Deserialize`
  id Int @id @default(autoincrement())
  /// User name
  /// @prs.skip
  name String?
  /// User emails
  emails String[]
  /// User age
  age Int? @default(0)
}

/// Post model with only defaults
/// @prs.derive = Debug,Clone,serde::Deserialize,serde::Serialize
model post {
  id Int @id @default(autoincrement())
  title String
  content Json
  published Boolean @default(false)
  publishedAt DateTime? @default(now())
}
```

Becomes:

```rust
#[doc = "User model documentation"]
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct User {
    #[doc = "User ID"]
    #[serde(rename = "user")]
    pub(crate) user_id: usize,
    #[doc = "User emails"]
    pub(crate) emails: Vec<String>,
    /// User age
    pub(crate) age: Option<i32>,
}

#[doc = "Post model with only defaults"]
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub content: serde_json::Value,
    pub published: bool,
    pub published_at: Option<chrono::DateTime<chrono::Utc>>,
}
```

## Constraints

Currently, it is up to **the user** to ensure all types have valid derive attributes. Specifically, if the `rename` attribute is needed, then `serde::Deserialize` and `serde::Serialize` must be used. The generator will not add them automatically.

## Development

```bash
npx prisma generate
```
