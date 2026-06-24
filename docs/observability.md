# Observability (T22)

This repo ships a self-contained observability substrate at
`crates/phenotype-journeys-observability/`. It wraps the crates.io `tracing`
+ `tracing-subscriber` stack directly so `cargo install --path .` is
hermetic — no external fleet dependency.

## Why the change?

The earlier implementation re-exported `pheno-tracing` from a pinned git
tag (`v0.4.0`). The upstream repo was archived and the tag is no longer
resolvable, which broke `cargo install --path .` with:

```
error: failed to find tag `v0.4.0`
```

The fix inlines the observability implementation. The CLI surface and the
`init_tracing` / `info!` / `instrument` re-exports are unchanged, so
existing call sites keep working.

## Quickstart (local)

```rust
use phenotype_journeys_observability::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing("phenotype-journey", "http://localhost:4317")?;
    info!(version = env!("CARGO_PKG_VERSION"), "starting");
    Ok(())
}
```

The endpoint argument is stored for downstream consumers; the default build
does not actively export to OTLP. Set `RUST_LOG=debug` to bump verbosity,
or `PHENOTYPE_JOURNEY_LOG_FORMAT=json` to switch the formatter to JSON
for log shippers.

## What's in the box

- `init_tracing(service_name, otlp_endpoint)` — install a process-wide
  subscriber. Honours `RUST_LOG` (falls back to
  `info,phenotype_journey=debug`). Idempotent: subsequent calls are a
  no-op.
- `info!`, `warn!`, `error!`, `debug!`, `trace!`, `instrument!`, `span!` —
  re-exports of the upstream `tracing` macros.
- `Counter` / `Histogram` / `RequestMetrics` — atomic-backed primitives
  for hand-rolled metrics without pulling in `prometheus` or `metrics`.
- `ServiceName` / `OtlpEndpoint` / `TracePort` / `TraceResult` — typed
  carriers preserved for downstream consumers.
- `SpanGuard` — RAII span guard so dispatchers can scope work without
  juggling `span.enter()` lifetimes.

## CI

`.github/workflows/observability-smoke.yml` builds the observability crate,
runs its unit tests, and verifies the release binary starts. It no longer
spins up an `otel-collector` container because the default build is local
only — exporters are an opt-in concern (see `Cargo.toml` for the `otlp`
feature flag once it's wired up).

## Out of scope

- Federation mTLS (ADR-046) — separate PR.
- Distributed tracing context propagation headers (B3 / W3C tracecontext)
  over the `reqwest` client — separate PR once gateway signing lands.
- Active OTLP gRPC export — the substrate is pluggable; consumers can
  layer `tracing-opentelemetry` on top without changing the public API.
