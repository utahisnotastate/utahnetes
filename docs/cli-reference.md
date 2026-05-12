# CLI reference

Command: **`utahnetes`**

Global behavior:

- Initializes `env_logger` with default filter **`info`** if `RUST_LOG` is unset.

---

## `ignite`

**Synopsis**

```text
utahnetes ignite [OPTIONS]
```

**Options**

| Option | Short | Default | Description |
|--------|-------|---------|-------------|
| `--capacity` | `-c` | `10000` | Scale for the toy thermodynamic meter |

**Examples**

```bash
utahnetes ignite
utahnetes ignite --capacity 5000
```

**See also:** [Tutorial: ignite](tutorial-ignite.md)

---

## `pour`

**Synopsis**

```text
utahnetes pour --file <PATH> [OPTIONS]
```

**Options**

| Option | Short | Default | Description |
|--------|-------|---------|-------------|
| `--file` | `-f` | *(required)* | Path to `.wasm` |
| `--observe-secs` | | `5` | Seconds to listen for remote heat gossip |

**Examples**

```bash
utahnetes pour --file ./demo.wasm
utahnetes pour -f ./demo.wasm --observe-secs 12
```

**See also:** [Tutorial: pour](tutorial-pour.md)

---

## Environment variables

| Variable | Purpose |
|----------|---------|
| `RUST_LOG` | Standard `env_logger` filter (e.g. `info`, `debug`, `utahnetes=debug`) |

---

## Topics (implementation detail)

Defined in `src/main.rs`:

- `utahnetes-heat`
- `utahnetes-pour`

See [Networking concepts](networking-concepts.md).
