# Networking concepts (LAN prototype)

Utahnetes uses **libp2p** building blocks. You do not need to master libp2p to use the CLI, but these terms explain “why it behaves like it behaves.”

---

## mDNS: “who is around me?”

**mDNS** (multicast DNS) is commonly used for local service discovery.

In Utahnetes, ignited nodes can discover peers on the same LAN without you manually typing IP addresses.

**Implication:** if your Wi‑Fi isolates clients (“AP isolation”), discovery may fail.

---

## gossipsub: “many-to-many bulletin board”

**Gossipsub** is a pub/sub protocol designed for decentralized networks:

- publishers post to **topics**,
- subscribers receive relevant messages,
- the network **gossips** messages outward so subscribers do not need a direct TCP link to the publisher.

Utahnetes uses two topics (constants in `main.rs`):

| Topic | Purpose |
|-------|---------|
| `utahnetes-heat` | text payloads like `HEAT:0.42` |
| `utahnetes-pour` | binary pour frames containing WASM bytes |

---

## TCP + Noise + Yamux (transport stack)

The Swarm builder configures:

- **TCP** for connectivity,
- **Noise** for encryption/handshake patterns used in many libp2p deployments,
- **Yamux** for stream multiplexing (“many conversations, one pipe”).

---

## “LAN” vs “internet”

The current documentation and defaults assume **local lab** conditions.

Running across the public internet typically requires:

- reachable listen addresses,
- NAT traversal strategies,
- security and abuse controls,

…which are not the focus of this prototype.

---

## Next

- [Tutorial: ignite](tutorial-ignite.md)
- [Security and privacy](security-and-privacy.md)
