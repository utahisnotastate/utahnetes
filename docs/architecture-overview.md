# Architecture overview (for readers of the Rust code)

This document maps **modules → responsibilities** as implemented in this repository.

---

## Crate layout

| Path | Responsibility |
|------|----------------|
| `src/lib.rs` | Crate entry; re-exports public types |
| `src/swarm/thermodynamics.rs` | `NodeThermodynamics` “heat” accounting |
| `src/swarm/network.rs` | `UtahnetesBehavior` (mDNS + gossipsub), `UtahnetesSwarmEvent` |
| `src/compute/mitosis.rs` | `WasmCell` (Wasmtime embed, `run` export) |
| `src/main.rs` | CLI + swarm builder + reactor loop + pour logic |

---

## Library public API (high level)

- **`NodeThermodynamics`**: load/heat helpers (`add_load`, `remove_load`, `current_heat_index`, `can_accept_liquid`).
- **`UtahnetesBehavior`**: constructs gossipsub + mdns behaviours; can `broadcast_heat_signature`.
- **`UtahnetesSwarmEvent`**: unified `to_swarm` event enum for downstream matching.
- **`WasmCell`**: `ingest_code`, `execute_intent`, `trigger_mitosis` (placeholder).

---

## CLI architecture

`main.rs` builds a libp2p swarm using `SwarmBuilder`:

1. `with_existing_identity(Keypair)`
2. `with_tokio()`
3. `with_tcp(..., noise::Config::new, yamux::Config::default)`
4. `with_behaviour(|key| UtahnetesBehavior::new(key.clone()))`

Then:

- subscribes to topics,
- listens on `0.0.0.0:0`,
- runs a `tokio::select!` loop for ignite,
- runs a timed observation loop for pour.

---

## Known simplifications (intentional)

- WASM migration / mitosis snapshotting is not implemented beyond a stub.
- Pour uses gossip payloads for WASM bytes (size capped).
- No persistent identity store across restarts (keys are generated per run in CLI paths shown).

---

## Extension ideas (roadmap fodder)

- request/response for large artifacts
- explicit peer bootstrapping lists for non-mDNS networks
- richer host imports for WASI workloads
- metrics + tracing (`tracing` crate) integration

See [Contributing and roadmap](contributing-and-roadmap.md).

---

## Related docs

- [Networking concepts](networking-concepts.md)
- [Tutorial: WASM cells](tutorial-wasm-cell.md)
