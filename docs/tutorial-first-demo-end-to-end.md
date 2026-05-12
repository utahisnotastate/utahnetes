# Tutorial: first demo end-to-end (copy/paste friendly)

This tutorial assumes two physical machines (**A** and **B**) on the **same LAN**, plus optionally a third (**C**) for pouring. You can sometimes use VMs, but Wi‑Fi AP isolation breaks many demos—**wired LAN is the most reliable**.

---

## 0) Preconditions (5 minutes)

- [Rust installed](installation.md) on each machine you will use for `pour` / building.
- Same subnet, no guest isolation.
- Agree on a **trusted** `.wasm` file built by someone you trust.

---

## 1) Build once per machine (or copy the binary)

On each machine:

```bash
cd utahnetes
cargo build --release
```

Binary path:

- Linux/macOS: `target/release/utahnetes`
- Windows: `target\release\utahnetes.exe`

---

## 2) Start two cups

**On machine A:**

```bash
RUST_LOG=info ./target/release/utahnetes ignite
```

**On machine B:**

```bash
RUST_LOG=info ./target/release/utahnetes ignite
```

Leave both running.

### What “good” looks like

Within a short time you should see:

- a **listen address** line on each,
- occasional **heat broadcast** lines,
- **mDNS** activity as peers discover each other (exact wording varies).

---

## 3) Wait for the mesh to “wake up”

Wait **10–30 seconds** before pouring. This reduces “no heat samples” situations.

---

## 4) Pour the WASM

**On machine C** (or A/B if you only have two machines—see note below):

```bash
RUST_LOG=info ./target/release/utahnetes pour --file ./path/to/app.wasm --observe-secs 8
```

### If you only have two machines

You can run `pour` from the same machine as one `ignite`, but mentally separate the processes:

- Terminal 1: `ignite`
- Terminal 2: `pour` (after ignite is healthy)

This still requires network stack behavior to cooperate; if it’s flaky, add a third machine.

---

## 5) Interpret the outcome

### Success patterns

- `pour` logs a published frame size.
- An `ignite` terminal logs acceptance / WASM execution (or errors if the module is incompatible).

### Common non-success patterns

- “No remote heat samples” → networking/timing; increase `--observe-secs`.
- WASM too large → see [Tutorial: pour](tutorial-pour.md).
- WASM missing `run` → see [Tutorial: WASM cells](tutorial-wasm-cell.md).

---

## 6) Shut down cleanly

Press **Ctrl+C** on each `ignite` terminal.

---

## Next

- [Homelab scenarios](homelab-scenarios.md) for more ideas
- [Troubleshooting](troubleshooting.md)
