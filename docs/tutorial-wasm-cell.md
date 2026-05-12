# Tutorial: WASM cells (`WasmCell`)

Utahnetes executes user code as **WebAssembly modules** through `WasmCell` (backed by Wasmtime in this repo).

---

## The contract (today)

Utahnetes expects a WASM module that exports a function:

- **name**: `run`
- **signature**: `() -> ()` (no parameters, no return values)

If `run` is missing, execution fails with an error from the embedding layer.

---

## Minimal “hello” style module (conceptual)

You typically compile from Rust/C/C++/TinyGo/etc. The exact build steps are language-specific; Utahnetes itself only sees **bytes**.

At a high level:

1. Produce a **`.wasm`** file.
2. Ensure it exports `run`.
3. `pour` it (CLI) or call `WasmCell` APIs (library).

---

## Execution model notes

- **Blocking execution**: the current prototype calls into Wasmtime directly. Very heavy workloads can block the async task; for production you would offload to a worker pool.
- **No automatic snapshotting yet**: `trigger_mitosis` is a placeholder hook for future “serialize and resume” style ideas.

---

## Size and safety expectations

- Treat WASM as **executable code**.
- Only run modules from **sources you trust**.
- Keep modules **small** for the prototype pour path (see [Tutorial: pour](tutorial-pour.md)).

---

## Library usage sketch (Rust)

```rust
use utahnetes::WasmCell;

let bytes = std::fs::read("app.wasm")?;
let cell = WasmCell::ingest_code(&bytes)?;
cell.execute_intent()?;
```

---

## Next

- [Security and privacy](security-and-privacy.md)
- [Architecture overview](architecture-overview.md)
