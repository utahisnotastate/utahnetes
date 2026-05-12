# Contributing and roadmap

Thank you for wanting to improve Utahnetes. This project is intentionally small; contributions that **preserve clarity** are valued over contributions that **maximize features**.

---

## Ways to contribute without writing Rust

- Improve documentation (especially real-world network pitfalls).
- Add **safe** demo WASM examples under `examples/` (if you add binaries, keep licensing clear).
- Write a short “lessons learned” doc from your LAN demo.

---

## Engineering contribution ideas (Rust)

### Reliability and ops

- graceful shutdown hooks
- structured logging (`tracing`) alongside `log`
- integration tests behind `#[cfg(feature = "...")]` to avoid flaky CI

### Networking

- optional static peer bootstrap (for networks without mDNS)
- configurable topic names via CLI flags / env vars

### WASM transport

- chunked pour/reassembly with timeouts
- optional content addressing (hash advertised, bytes fetched via side channel)

### Safety

- explicit “trust store” / allowlist for WASM hashes
- execution timeouts and fuel limits in Wasmtime

---

## Project values

- **Teachability** over feature count.
- **Honest scope** over marketing claims.
- **Local lab friendliness** over global internet readiness (unless explicitly designed).

---

## Development commands

```bash
cargo fmt
cargo clippy
cargo test
cargo build --release
```

---

## Roadmap (non-binding)

1. Stable CLI configuration file (optional).
2. Safer defaults for WASM execution (fuel, timeout).
3. Better pour transport for large modules.
4. Optional web dashboard **out of tree** (separate crate) if someone wants it.

---

## Code of conduct

If this repository grows collaboration, adopt a standard Contributor Covenant CoC and link it here.

---

## Questions

Open discussions in your fork or upstream tracker as appropriate.
