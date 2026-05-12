# Tutorial: `utahnetes ignite`

`ignite` starts a **long-lived Utahnetes node** (“a cup on the table”) that:

- joins a **libp2p swarm** with TCP + Noise + Yamux,
- advertises itself on the LAN via **mDNS**,
- subscribes to gossipsub topics for **heat** and **pour**,
- periodically **publishes** its local heat index,
- may **execute WASM** when a pour message is addressed to it (or “any cold cup” mode).

---

## Command

```bash
RUST_LOG=info utahnetes ignite
```

### Options

| Flag | Meaning | Default |
|------|---------|--------|
| `-c`, `--capacity` | Internal scale for the toy thermodynamic meter | `10000` |

Example:

```bash
utahnetes ignite --capacity 20000
```

`capacity` affects the **interpretation** of load counters in `NodeThermodynamics` (the “max cup size” metaphor). It does not magically change your CPU hardware.

---

## What you should see (happy path)

Typical `info` logs include:

- **Listening address** after `listen_on` succeeds (often includes a high ephemeral TCP port).
- Periodic lines about **broadcast heat index** (a percentage-like display of the 0.0–1.0 heat index).
- **mDNS** lines as peers appear/disappear on the LAN.
- If another peer gossips heat, you may see **peer heat** lines when messages arrive.

---

## Mental model: what `ignite` is doing while it runs

1. **Transport**: encrypted TCP connections to other peers (Noise + Yamux).
2. **Discovery**: mDNS helps local peers find each other without a static peer list.
3. **Topics**:
   - `utahnetes-heat` — text payloads like `HEAT:0.37`
   - `utahnetes-pour` — binary frames described in [Tutorial: pour](tutorial-pour.md)
4. **Thermodynamics**: the node tracks a simple load counter and refuses new pour executions when “too hot” (heat index ≥ ~85% in the current logic).

---

## Common operator mistakes

### Forgetting to keep the machine awake

Laptops sleeping mid-demo breaks discovery and gossip continuity.

### Running on guest Wi‑Fi that isolates clients

Some guest networks prevent device-to-device communication. Utahnetes needs peers to talk to each other.

### Running only `ignite` and expecting magic clustering across the internet

This prototype is oriented to **LAN** discovery patterns.

---

## Stopping the node

Press **Ctrl+C**. There is no graceful shutdown tutorial in the prototype beyond OS signal handling.

---

## Next

- [Tutorial: pour](tutorial-pour.md) (send work to ignited nodes)
- [Networking concepts](networking-concepts.md)
- [Troubleshooting](troubleshooting.md)
