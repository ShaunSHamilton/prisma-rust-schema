# Changelog

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
