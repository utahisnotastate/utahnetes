# Troubleshooting

Symptoms first, then likely causes and fixes.

---

## Build failures on Windows (policy / blocked build scripts)

**Symptoms**

- errors mentioning **Application Control**, **os error 4551**, or blocked `build-script-build`.

**Likely cause**

Corporate or consumer security policy blocking compiled build scripts.

**Mitigations**

- Build in **WSL2** or Linux VM/CI, copy artifacts back, or
- ask IT to allowlist Rust/Cargo build outputs (path-specific).

See [Installation](installation.md).

---

## `ignite` runs but I never see peers / mDNS seems dead

**Likely causes**

- **AP/client isolation** on Wi‑Fi.
- Devices on **different VLANs** without multicast routing.
- **Firewall** blocking local multicast or TCP high ports.

**Mitigations**

- use wired Ethernet on the same switch,
- disable isolation on lab Wi‑Fi,
- temporarily relax firewall rules for the demo subnet (with IT approval).

---

## `pour` says “No remote heat samples yet”

**Likely causes**

- no other ignited peers reachable,
- observation window too short (`--observe-secs`),
- network segmentation.

**Mitigations**

- start two `ignite` processes on reachable machines,
- increase `--observe-secs`,
- verify mDNS works (see above).

---

## `pour` fails: WASM exceeds pour limit

**Cause**

Prototype pour embeds WASM in gossip frames; there is a hard cap (~480 KiB).

**Mitigations**

- build a smaller module,
- implement a chunked side protocol (future work),
- use a different distribution mechanism for large artifacts.

---

## WASM run fails / ingest fails

**Common causes**

- missing export **`run`** with `() -> ()` signature,
- module needs WASI imports not provided by the host,
- invalid WASM bytes.

**Mitigations**

- validate module locally with wasm tooling your team prefers,
- simplify module to a known-good hello world.

See [Tutorial: WASM cells](tutorial-wasm-cell.md).

---

## Logs are too quiet

Set:

```bash
RUST_LOG=debug utahnetes ignite
```

---

## Still stuck?

Capture:

- OS + network type (Wi‑Fi vs wired),
- exact command lines,
- relevant log excerpts (redact IPs if needed),

…and open a discussion/issue in your fork’s tracker.

---

## Next

- [Networking concepts](networking-concepts.md)
- [FAQ](faq.md)
