Product Requirements

  - Objective
    Deliver first-class Tokio integrations so miniFIX  users
  can wire FIX sessions with popular async tooling (tokio,
  tokio-util) instead of juggling generic futures utilities.
  - Scope
      - crates/minifix/src/session/event_loop.rs
      - crates/minifix/src/session/connection.rs
      - crates/minifix/src/tagvalue/tokio_decoder.rs
      - Tokio-oriented orchestration glue (new module or
  feature flag inside crates/minifix/src/session)
  - Target Changes
      - Add a Tokio-specific event loop that wraps
  LlEventLoop behaviour using tokio::io::AsyncReadExt,
  tokio::select!, and tokio::time::{sleep, sleep_until}; drop
  the tokio_util::compat layer in tests.
      - Provide a Tokio flavour of FixConnection that
  talks directly to tokio::net::TcpStream, uses Tokio I/
  O traits, and optionally exposes channel-based hooks
  (tokio::sync::mpsc) for app callbacks.
      - Rework tagvalue::tokio_decoder into a ready-to-
  use tokio_util::codec::Decoder (and optional Encoder)
  using split_to/advance so it cooperates with FramedRead/
  FramedWrite; revive the commented TokioDecoder.
      - Publish a top-level helper (feature-gated utils-
  tokio) that ties decoder, encoder, and connection lifecycle
  together for Tokio runtimes.
  - Non-Goals
      - No removal of the executor-agnostic futures
  implementation.
      - No protocol or behaviour changes to FIX encoding/
  decoding.
      - No new persistence, logging, or application-layer
  logic.
  - Constraints
      - Maintain backwards compatibility: new Tokio pieces
  must be additive behind existing feature flags.
      - Avoid duplicating business logic; wrap or delegate to
  the current decoder/event loop where possible.
      - Keep public APIs opt-in via utils-tokio or new
  feature flag; document gating clearly.
  - Risks & Mitigations
      - Divergence between generic and Tokio paths: factor
  common logic into shared helpers and test both modes.
      - Potential buffer management bugs in the codec: cover
  with integration tests feeding partial frames and malformed
  data.
      - Increased maintenance burden: document expectations
  and ensure CI exercises Tokio code (feature-enabled test
  job).
  - QA / Validation
      - cargo fmt, cargo clippy, cargo test --workspace with
  and without --features utils-tokio.
      - Add async integration tests that drive TokioDecoder
  via tokio_util::codec::FramedRead.
      - Manual smoke test: connect two Tokio tasks exchanging
  FIX heartbeats through the new helpers.
  - Rollout
      - Implement behind existing utils-tokio flag (add new
  flag if needed).
      - Update docs (README, docs/) to show Tokio usage
  examples.
      - No migration notes required; advertise new APIs in
  release notes.
  - Success Metrics
      - Users can compose a working Tokio FIX client with <
  ~20 LOC of glue (examples/docs).
      - Removal of tokio_util::compat in tests; new Tokio
  tests pass on CI.
      - Increased adoption of utils-tokio feature (optional
  telemetry or anecdotal feedback).
