# Changelog

## [1.0.3] - 2025-07-21

### Fixed

- Stop trying to derive default for enums

## [1.0.2] - 2025-07-17

### Fixed

- Adjust version requirements for dependencies

## [1.0.1] - 2025-07-17

### Fixed

- `type` keyword names is auto-transformed to `_type`

## [1.0.0] - 2025-07-15

### Added

- `schema_path` now accepts anything implementing [`IntoUrl`](https://docs.rs/reqwest/latest/reqwest/trait.IntoUrl.html)

## [0.7.0] - 2025-07-10

### Added

- `patch` feature

```rust
import_types!(
  schema_path = "./prisma/schema.prisma",
  patch = [
    struct MyStruct {
      #[serde(default)]
      pub patched_field: String
    },
    struct AnotherStruct {
      #[serde(rename = "patch")]
      pub patched_field: String
    }
  ]
)
```

### Fixed

- `prefix` now correctly prefixes type values for composite types

## [0.6.0] - 2025-07-02

### Changed

- `Int` converts to `i64` instead of `i32`
- `Float` converts to `f64` instead of `f32`

## [0.5.0] - 2025-06-04

### Changed

- Ignore relation fields including model references within models

## [0.4.0] - 2025-06-03

### Added

- Import options for `import_types!` macro
- `derive` option
- `include` option
- `prefix` option

## [0.3.0] - 2025-05-29

### Changed

- Remove library exports of prisma internal TS types
- Convert to a proc-macro crate

### Added

- `import_types!` macro to import Prisma schema types
- License text

## [0.2.1] - 2025-04-30

### Fixed

- Moved `bson` dependency to `dev-dependencies`

## [0.2.0] - 2025-04-22

### Added

- Support for `@prs.rename` to rename fields, structs, enums, and enum values.
- Support for `@prs.skip` to skip fields, structs, enums, and enum values.
- Support for `@prs.visibility` to set the visibility of structs, fields, and enums.
- Support for `@prs.type` to specify the type of a struct field.
- Support for `@prs.derive` to specify derive attributes for structs and enums.

### Changed

- Defaults have been removed for derive attributes
- Output file no longer imports (`use`) any crates
  - Everything must be fully qualified

## [0.1.1] - 2025-04-19

### Added

- Library of Prisma internal TypeScript types.

## [0.1.0] - 2025-04-17

### Added

- Initial release of the project.
- `output` option to specify the output file name.
