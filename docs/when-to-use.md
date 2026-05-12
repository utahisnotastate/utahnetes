# When to use Utahnetes (and when not to)

This page is a **decision guide**. Utahnetes is interesting precisely because it is small; small tools shine in the right context and become dangerous in the wrong one.

---

## Green lights: Utahnetes is a good fit

### Education and intuition-building

- Teaching **peer discovery** and **pub/sub** without standing up cloud infrastructure.
- Demonstrating **WASM as a distribution format** for tiny tools.
- Giving students a **bounded** system that fits in one screen of architecture.

### Trusted-LAN experiments

- Homelab nights, hackathons, meetups where everyone consents and the network is controlled.
- Prototyping “intent-like” workflows where humans coordinate timing manually.

### Integration spikes (with engineering support)

- A spike to answer: “Could we gossip lightweight health signals between nodes?”
- A spike to answer: “Could we ship WASM modules for ultra-small utilities?”

Utahnetes gives you a **reference implementation** to copy ideas from, not a mandate to copy code verbatim.

---

## Yellow lights: proceed, but add constraints

### You need slightly more realism

Add checklists:

- explicit participant list,
- logging retention,
- documented WASM sources,
- a rollback plan.

### You have multiple VLANs or Wi‑Fi client isolation

mDNS discovery may not cross some network boundaries. Utahnetes may still work if you manually connect peers (future feature area), but **out-of-the-box LAN discovery might not**.

### You want bigger binaries than the prototype pour limit

Today’s `pour` path embeds WASM in a gossipsub message with an intentionally capped size (see [Tutorial: pour](tutorial-pour.md)). If you need large artifacts, you need a different transport design (chunking, object storage, HTTP side channel, etc.).

---

## Red lights: do not use Utahnetes for this (without major redesign)

### Untrusted users and untrusted WASM

Running WASM is **not** the same as “safe to run malware.” Treat unknown WASM like unknown executables.

### Production customer data

No built-in encryption-at-rest for payloads, no built-in tenant isolation story, no compliance controls.

### Mission-critical uptime

This prototype does not include HA control planes, rolling upgrade orchestration, or SLO tooling.

### Internet-wide open swarms

The defaults and documentation assume **local lab conditions**. Opening this to the public internet would require hardening, authentication, rate limits, and operational maturity far beyond this repo’s scope.

---

## “Is Utahnetes right for my startup?”

Ask this instead:

> “Are we trying to learn and prototype coordination ideas quickly on a trusted LAN with engineering supervision?”

- If **yes**: Utahnetes might be a fun internal tool or demo backbone.
- If **no**: use mature orchestration and come back to Utahnetes as a learning reference.

---

## Related docs

- [Non-technical guide](non-technical-guide.md)
- [Security and privacy](security-and-privacy.md)
- [Homelab scenarios](homelab-scenarios.md)
