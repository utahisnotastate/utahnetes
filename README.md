# Utahnetes

**Repository:** [github.com/utahisnotastate/utahnetes](https://github.com/utahisnotastate/utahnetes)

**Utahnetes** is an experimental Rust project that explores a *different mental model* for running software across several computers: instead of one central “boss” telling every machine exactly what to do (the way many cluster systems feel), participating machines share simple **signals** (like “how busy am I?”) and can accept **small programs** shipped as **WebAssembly** when they have spare capacity.

If that sentence already feels like a lot, start here: **[Non-technical guide: what you can actually do](docs/non-technical-guide.md)**. It is written for people who want to be productive, run demos, or collaborate with a technical teammate without becoming a programmer first.

---

## Documentation map (read in any order)

| You are… | Start here |
|-----------|----------------|
| **Not technical / “I just want to understand it”** | [Non-technical guide](docs/non-technical-guide.md) |
| **Deciding if this fits your problem** | [When to use Utahnetes](docs/when-to-use.md) |
| **Installing and running the first time** | [Installation](docs/installation.md) |
| **Running a long-lived “cup” on the network** | [Tutorial: `ignite`](docs/tutorial-ignite.md) |
| **Sending a WASM program into the swarm** | [Tutorial: `pour`](docs/tutorial-pour.md) |
| **Understanding WASM “cells”** | [Tutorial: WASM cells](docs/tutorial-wasm-cell.md) |
| **Homelab, classroom, or demo ideas** | [Scenarios cookbook](docs/homelab-scenarios.md) |
| **Confused by the words** | [Glossary](docs/glossary.md) |
| **Something broke** | [Troubleshooting](docs/troubleshooting.md) |
| **Safety and trust on a LAN** | [Security and privacy](docs/security-and-privacy.md) |
| **How the pieces connect (technical)** | [Architecture overview](docs/architecture-overview.md) |
| **Common questions** | [FAQ](docs/faq.md) |
| **CLI flags and env vars** | [CLI reference](docs/cli-reference.md) |
| **First successful demo (step-by-step)** | [First demo end-to-end](docs/tutorial-first-demo-end-to-end.md) |
| **Managers / teachers** | [For managers and educators](docs/for-managers-and-educators.md) |
| **Printable host checklists** | [Non-technical checklists](docs/non-technical-checklists.md) |
| **Kubernetes comparison (teaching)** | [Comparing to Kubernetes](docs/tutorial-comparing-to-kubernetes.md) |
| **Contributing / roadmap** | [Contributing and roadmap](docs/contributing-and-roadmap.md) |
| **Full tutorial index** | [docs/README.md](docs/README.md) |

---

## What this repository contains (one paragraph)

- A **Rust library** (`utahnetes`) with three main ideas: a simple **“heat” meter** for local load, a **peer-to-peer network layer** (discovery via mDNS, messaging via gossipsub), and a **WebAssembly runner** that can execute a module export named `run`.
- A **command-line tool** with two commands:
  - **`utahnetes ignite`**: join the local swarm, listen, gossip heat, and optionally execute incoming WASM “pour” payloads.
  - **`utahnetes pour`**: read a `.wasm` file, listen briefly for peer heat signals, then publish a pour message so a suitable machine can execute it.

This is **research / prototype** quality: excellent for learning, demos on a trusted LAN, and hacking; **not** a drop-in replacement for production orchestrators without additional engineering.

---

## Quick start (technical, minimal)

Prerequisites: [Rust toolchain](https://rustup.rs/) (stable).

```bash
# Clone and enter the repo, then:
cargo build --release

# Terminal A (a “cup” that can receive work):
RUST_LOG=info ./target/release/utahnetes ignite

# Terminal B (after A is running; optionally more terminals with ignite):
RUST_LOG=info ./target/release/utahnetes pour --file ./path/to/app.wasm
```

On Windows PowerShell, set logging like:

```powershell
$env:RUST_LOG = "info"
cargo run --release -- ignite
```

See [Installation](docs/installation.md) for platform notes (including Windows Application Control issues some users hit when building dependencies).

---

## Honest boundaries (read this once)

Utahnetes is **not** claiming to replace Kubernetes, Nomad, or cloud platforms today. It **is** a compact story + working code about:

- decentralized discovery on a LAN,
- gossip-style messaging,
- a playful “thermodynamics” metaphor for load,
- and WASM execution hooks.

If you need multi-region orchestration, strong isolation guarantees, secrets management, persistent volumes, and compliance-ready operations, you should use mature platforms—and optionally **borrow ideas** from Utahnetes for prototypes.

---

## License

Dual-licensed under **MIT OR Apache-2.0** (see `LICENSE` files if present in your fork; otherwise follow your org’s standard licensing for this template).

---

## Contributing

See [Contributing and roadmap](docs/contributing-and-roadmap.md) for how to extend the prototype (chunked WASM transport, richer intents, metrics, etc.).
