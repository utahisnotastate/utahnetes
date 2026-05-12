# Tutorial: `utahnetes pour`

`pour` is the “send work into the swarm” command. In this prototype it:

1. reads a **`.wasm`** file from disk,
2. starts a temporary swarm participant,
3. listens for **heat gossip** for a configurable time window,
4. chooses a **targeting strategy**,
5. publishes a **binary pour frame** on the `utahnetes-pour` gossip topic.

---

## Command

```bash
RUST_LOG=info utahnetes pour --file ./path/to/app.wasm
```

### Options

| Flag | Meaning | Default |
|------|---------|--------|
| `-f`, `--file` | Path to the WASM file | *(required)* |
| `--observe-secs` | How long to listen for `HEAT:` gossip before deciding | `5` |

Example:

```bash
utahnetes pour --file ./cells/demo.wasm --observe-secs 8
```

---

## Size limits (important)

This prototype embeds WASM bytes directly in a gossipsub payload. There is an intentional cap:

- **Maximum WASM size**: about **480 KiB** (documented in code as `MAX_POUR_WASM`)

If your WASM is larger, `pour` will fail fast with an error telling you it exceeds the pour limit.

**Why the limit exists:** gossip protocols are great for small control-plane messages; they are a poor default transport for large artifacts.

**What to do for bigger apps:** use object storage, chunking, or request/response transports (roadmap territory).

---

## Targeting modes (how “coldest cup” works)

### Coldest peer mode (default when heat samples exist)

While observing, `pour` records remote peers’ heat values parsed from messages on `utahnetes-heat`.

If it sees at least one remote sample, it picks the peer with the **lowest heat** and sends a **targeted** pour frame (`UTPO`).

### “Any cold cup” mode

If **no** remote heat samples arrive in time, `pour` logs a warning and uses **any** mode (`UTPA`), meaning:

> “The first sufficiently ‘cold’ ignited node that accepts the message may run it.”

This keeps demos moving on flaky networks, but it is less precise.

---

## Pour frame format (for integrators)

This is implementation-defined and may change in future versions.

- **`UTPA`**: `UTPA` + `u32` little-endian WASM length + raw WASM bytes  
- **`UTPO`**: `UTPO` + `u16` big-endian peer id string length + UTF-8 `PeerId` string + `u32` LE WASM length + raw WASM bytes  

Igniting nodes decode these frames and decide whether to execute.

---

## Expected log patterns

- During observation, you may see little if the network is quiet.
- If no remote heat is seen: **“No remote heat samples yet; pouring in ANY-cold mode”**.
- After publishing: **“Pour frame published…”** and a short sleep to allow propagation.

---

## A reliable demo sequence

1. Machine A: `ignite`
2. Machine B: `ignite`
3. Wait ~10 seconds (mDNS + gossip warm-up)
4. Machine C: `pour --file tiny.wasm --observe-secs 6`

If everything is on the same LAN segment, C is more likely to observe heats and choose a coldest peer.

---

## Next

- [Tutorial: WASM cells](tutorial-wasm-cell.md)
- [Networking concepts](networking-concepts.md)
- [Troubleshooting](troubleshooting.md)
