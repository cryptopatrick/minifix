Product Requirements

  - Objective
    Deliver production-grade Simple Binary Encoding support
  for FIX 4.4 and FIX 4.2 so MiniFix users can encode/decode
  those dictionaries with SBE, alongside existing tag-value
  tooling.
  - Scope
      - New minifix-sbe-codegen crate to turn FIX 4.4/4.2 SBE
  XML schemas into Rust code.
      - Runtime encoder/decoder modules under crates/minifix/
  src/sbe/ (feature-gated).
      - Integration hooks with SOFH (minisofh) and session
  layers to route SBE frames.
      - Documentation and examples covering both FIX
  versions.
  - Target Changes
      - Parse the official FIX Trading Community IR templates
  for FIX 4.4 and FIX 4.2 (both BE/LE variants).
      - Generate zero-copy flyweight types (static offsets,
  typed getters/setters, repeating-group helpers).
      - Provide SbeEncoder/SbeDecoder APIs mirroring
  tagvalue::{Encoder,Decoder}, including streaming
  decode, validation, and conversion to existing FieldMap
  abstractions where feasible.
      - Extend SOFH handling so
  EncodingType::SimpleBinaryEncodingV10{BE,LE} dispatches to
  generated codecs.
      - Ship examples/ demonstrating encode/decode for
  canonical FIX 4.4/4.2 messages; update docs with usage
  instructions.
  - Non-Goals
      - No support for FIX versions beyond 4.2/4.4 in the
  initial release.
      - No dynamic schema loading at runtime (codegen is
  build-time only).
      - No automatic bridging between SBE and tag-value
  beyond helper conversions.
  - Constraints
      - Must remain additive and feature-gated (e.g.
  --features sbe).
      - Generated code must be no_std-friendly where possible
  and avoid allocations on hot paths.
      - Follow FIX SBE spec precisely, including endianness,
  header layout, presence maps, and repeating-group
  semantics.
      - Maintain compatibility with existing MiniFix traits
  (e.g. reuse Buffer, SetField where practical).
  - Risks & Mitigations
      - Schema drift: changes in official templates could
  break codegen. Mitigate by versioning generated artifacts
  and adding CI checks on upstream schema hashes.
      - Performance regressions: ensure micro-benchmarks
  compare favourably to tag-value; profile with criterion and
  optimize hot loops.
      - Interoperability bugs: create conformance tests
  against reference data captured from other FIX engines/
  tools; add fuzzing for malformed frames.
      - Maintenance burden: keep generator modular, document
  template coverage, and add snapshot tests for generated
  code.
  - QA / Validation
      - cargo test --all-features covering generated
  FIX 4.4/4.2 suites (round-trip encode/decode, boundary
  cases).
      - cargo fmt, cargo clippy enforced for generated and
  hand-written code.
      - Integration tests wiring SOFH + Tokio pipeline (if
  utils-tokio enabled) to exchange SBE frames.
      - Optional fuzz targets on decoder entry points.
      - Benchmark suite verifying throughput and allocation
  counts versus baseline goals.
  - Rollout
      - Stage 1: feature flag sbe with codegen CLI + runtime
  modules; publish beta docs.
      - Stage 2: add official examples and announce in
  release notes; solicit feedback.
      - Stage 3: once stable, keep sbe feature optional but
  documented as supported for production use.
  - Success Metrics
      - Successful round-trip tests for all FIX 4.4/4.2 SBE
  templates (100% coverage).
      - Benchmarks demonstrate ≥2× throughput vs tag-value
  for equivalent messages.
      - Positive feedback/adoption from early users (tracked
  via issues/discussions).
      - No interoperability regressions reported against
  external FIX engines using standard schemas.
