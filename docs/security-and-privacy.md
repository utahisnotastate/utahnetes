# Security and privacy (prototype LAN tool)

Utahnetes is educational software. Treat it like **running a local demo service**: powerful on a trusted LAN, inappropriate as a public-facing “compute marketplace” without major hardening.

---

## Threat model (intended)

- **Participants**: you and collaborators you trust.
- **Network**: a home lab, classroom VLAN, or office segment where device-to-device traffic is expected.
- **Goal**: learn coordination patterns; run small WASM demos.

---

## Threat model (NOT intended)

- Anonymous users on the internet submitting arbitrary WASM.
- Multi-tenant environments with regulated data.
- Hostile LANs (coffee shop Wi‑Fi) where any peer may speak to you.

---

## WASM is executable code

Even though WASM is sandboxed relative to native code in important ways, you should still assume:

- it can consume CPU and memory,
- it may be able to call into host capabilities depending on imports and embedding,
- bugs can still cause denial-of-service style issues.

**Rule:** only run modules from sources you trust, built by people you trust.

---

## Gossip is not a private diary

Messages on gossipsub topics are visible to participating peers in the mesh (by design).

Do not pour:

- secrets,
- credentials,
- personal health data,

…into Utahnetes topics without additional encryption and access control (not provided here).

---

## Noise encryption is not “automatic zero trust”

Transport encryption protects many on-wire attacks, but it does not magically validate that a peer is **morally** your friend—only that you completed a cryptographic handshake appropriate to the protocol configuration.

---

## Practical checklist before a demo

- Use a **dedicated demo network** if possible.
- Ensure **client isolation** is disabled on the Wi‑Fi AP for LAN demos.
- Use **separate accounts** / VMs if you are experimenting with untrusted WASM.
- Capture logs (`RUST_LOG=info`) for postmortems.

---

## Next

- [When to use Utahnetes](when-to-use.md)
- [Troubleshooting](troubleshooting.md)
