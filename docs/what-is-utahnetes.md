# What is Utahnetes?

Utahnetes is a **small Rust crate** plus a **CLI** that demonstrates a particular style of distributed execution:

1. **Machines discover each other on a local network** using mDNS (a “who is here?” mechanism).
2. **Machines share lightweight messages** using gossipsub (a “many-to-many bulletin board” style protocol).
3. **Each machine exposes a simple numeric “heat” signal** that stands in for “how busy / how full” it is (`NodeThermodynamics`).
4. **Work arrives as WebAssembly bytes** that can be executed locally (`WasmCell`), triggered by the CLI’s `pour` command in this prototype.

The name and narrative play with “liquid compute” and “thermodynamics” as **pedagogical metaphors**. The implementation is real; the metaphors are there to help humans reason about load and flow.

---

## What you can do with it today (literal capabilities)

### As a library (`utahnetes`)

You can embed:

- **Thermodynamics-style load accounting** (`NodeThermodynamics`): track a numeric load, compute a 0.0–1.0 “heat index,” and ask whether the node should accept more work.
- **A libp2p “fabric”** (`UtahnetesBehavior`): mDNS discovery + gossipsub publishing/subscribing.
- **A WASM runner** (`WasmCell`): compile bytes into a module and call an exported function named `run` with signature `() -> ()`.

### As a CLI (`utahnetes` binary)

- **`ignite`**: run a long-lived process that joins the LAN swarm, subscribes to topics, periodically gossips heat, and can execute pour payloads.
- **`pour`**: read a `.wasm` file, observe peer heat messages for a configurable window, choose a target strategy (coldest peer, or “any cold cup”), then publish a pour frame.

See [Tutorial: ignite](tutorial-ignite.md) and [Tutorial: pour](tutorial-pour.md).

---

## What Utahnetes is *not*

- **Not** a full “serverless platform” with auth, quotas, billing, autoscaling policies, and multi-region routing.
- **Not** a verified sandbox for running untrusted code from strangers. WASM is a container for bytecode; **trust still matters**.
- **Not** a complete migration path away from Kubernetes without substantial additional design.

---

## Mental model: three nouns

| Noun | Think of it as… | In code |
|------|-------------------|---------|
| **Cup** | A computer willing to participate | A process running `ignite` |
| **Heat** | A coarse “how busy am I?” dial | `NodeThermodynamics` + periodic gossip |
| **Liquid cell** | A small WASM package | `.wasm` bytes executed by `WasmCell` |

---

## Relationship to Kubernetes (one paragraph, no dunking)

Kubernetes solves a huge set of production problems with a large control plane and rich APIs. Utahnetes is not competing on feature parity; it is a **compressed teaching artifact** that highlights:

- decentralized discovery,
- epidemic messaging,
- packaging compute as WASM,

…and invites you to ask: “Which parts of my real system actually need a central planner, and which parts are just habit?”

---

## Next reading

- [When to use Utahnetes](when-to-use.md)
- [Networking concepts](networking-concepts.md)
- [Architecture overview](architecture-overview.md)
