# FAQ

## Is Utahnetes a Kubernetes replacement?

No. It is a **small educational prototype** with different goals. Kubernetes solves a large production problem set; Utahnetes demonstrates a few coordination ideas in a compact codebase.

---

## Do I need Kubernetes experience?

No. Basic command-line comfort helps. Understanding “LAN vs cloud” helps more than YAML expertise.

---

## Can my parents run it safely?

Only if they understand it runs **executable code** and they only use **trusted WASM** on a **trusted network**. For most households: treat it as a **hobbyist engineering toy**, not a consumer appliance.

---

## Why is WASM limited in size?

Gossipsub is tuned for relatively small control-plane messages. Large artifacts should use a different transport path (not implemented in this prototype).

---

## Can I use this on the public internet?

Not recommended without substantial security engineering (authentication, authorization, abuse controls, transport hardening, observability).

---

## What does “heat” actually measure?

A toy counter stored in `NodeThermodynamics`. It is **not** a hardware temperature sensor in the default code path. It is a metaphor + simple accounting hook.

---

## Why `run` specifically?

The prototype embedding calls an export named `run` with no parameters. That is a deliberate simplification.

---

## Can multiple ignites run on one machine?

You can try multiple processes; ports and peer identities differ. Discovery behavior may be confusing—prefer multiple physical machines for demos.

---

## Does it support Docker?

This repo does not orchestrate Docker containers. The “cell” story is WASM-oriented.

---

## Where do I get a `.wasm` file?

From a developer who builds one, or from your own Rust/C/TinyGo toolchain. Utahnetes does not ship a curated WASM app store.

---

## Is there a GUI?

Not in this repository. The CLI is the interface.

---

## What license applies?

The project declares **MIT OR Apache-2.0** in `Cargo.toml`. Add `LICENSE-MIT` / `LICENSE-APACHE` files if you are publishing formally.

---

## More reading

- [When to use Utahnetes](when-to-use.md)
- [Troubleshooting](troubleshooting.md)
