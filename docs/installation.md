# Installation

Utahnetes is a standard Rust project. You build the **library** and **binary** together.

---

## Prerequisites

- **Rust** (stable recommended), via [rustup](https://rustup.rs/).
- A **network interface** (Wi‑Fi or Ethernet) for LAN features.
- For demos: **two or more machines** on the same broadcast domain work best (or multiple terminals on one machine for partial testing—discovery behavior may differ).

---

## Clone and build

```bash
git clone <YOUR_REPO_URL> utahnetes
cd utahnetes
cargo build --release
```

Artifacts:

- Binary: `target/release/utahnetes` (Unix) or `target\release\utahnetes.exe` (Windows)
- Library: consumed by Rust code as the `utahnetes` crate

Run from source without installing:

```bash
cargo run --release -- ignite
cargo run --release -- pour --file ./something.wasm
```

---

## Logging

Utahnetes uses `env_logger`. Common settings:

```bash
RUST_LOG=info ./target/release/utahnetes ignite
RUST_LOG=debug ./target/release/utahnetes pour --file ./something.wasm
```

**Windows PowerShell:**

```powershell
$env:RUST_LOG = "info"
cargo run --release -- ignite
```

Default (if unset) is **`info`** for the binary’s initialization code path.

---

## Windows note: Application Control / Smart App Control

Some Windows environments block running **build scripts** from dependencies (you may see errors like “Application Control policy has blocked this file”).

Mitigations (pick what matches your organization’s policy):

- Build on **WSL2** (Linux) and run Linux binaries, or
- Use a **CI Linux runner** to produce artifacts, or
- Adjust **Windows Defender Application Control** allowlists for Rust/Cargo build outputs (IT-assisted).

This is an environment restriction, not a Utahnetes source bug.

---

## Optional: `cargo install` from path

If you maintain a fork:

```bash
cargo install --path .
```

Then ensure `~/.cargo/bin` is on your `PATH`.

---

## Verify the build

```bash
cargo check
```

For a stricter check:

```bash
cargo test
```

(Tests may be limited; if `cargo test` fails on Windows due to policy blocks, see the Windows note above.)

---

## Next steps

- [Tutorial: ignite](tutorial-ignite.md)
- [Tutorial: pour](tutorial-pour.md)
- [Troubleshooting](troubleshooting.md)
