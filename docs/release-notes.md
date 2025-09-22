# Next release

- Reduced number of dependencies in the `minisofh` crate.
- Reduce number of and rename public entities in `minifix::codegen`.
- Buffered decoders are now known as streaming decoders. Please see the new `StreamingDecoder` trait.
- Now using `u32` for tags rather than `u16`. This allows using user-defined tags with large values.
- `FieldValueError` for easier missing field detection. `FieldMap` method
  signatures changed to use this new `enum`.
- Improved repeating group logic and bug fixes (https://github.com/ferrumfix/ferrumfix/issues/12).
- Fix: reusing `Decoder` instances might cause decoding errors (https://github.com/ferrumfix/ferrumfix/issues/17).

# v0.7.0 (YYYY-MM-DD)

This release addresses the following issues:

- **New Cargo features with optional dependencies**. We simplified the dependency graph and hid "nice-to-have's" behing `utils-*` features. At the moment, the following utility features are available:
  - `utils-bytes`
  - `utils-chrono`
  - `utils-decimal`
  - `utils-openssl`
  - `utils-rust-decimal`
  - `utils-slog`
  - `utils-tokio`
